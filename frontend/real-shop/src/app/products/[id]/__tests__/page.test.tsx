import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import "@testing-library/jest-dom"
import { fireEvent, render, screen, waitFor, waitForElementToBeRemoved } from "@testing-library/react"
import { HttpResponse, delay, http } from "msw"

import {
  Image,
  InventoryPolicy,
  Media,
  MediaStatus,
  ProductStatus,
  Variant,
  getGetProductMockHandler,
} from "@/generated/backend"
import { server } from "@/mocks/server"

import ProductPage from "../page"

// JSDOMのscrollIntoViewをモック
window.HTMLElement.prototype.scrollIntoView = jest.fn()

// モック商品データ
const mockImage: Image = {
  id: "image1",
  src: "/images/test-image.jpg",
  alt: "テスト画像",
}

const mockMedia: Media = {
  id: "media1",
  status: MediaStatus.Active,
  content: {
    image: mockImage,
  },
  created_at: "2024-01-01T00:00:00Z",
  updated_at: "2024-01-01T00:00:00Z",
}

const mockVariant: Variant = {
  id: "variant1",
  available_for_sale: true,
  list_order: 1,
  inventory_item_id: "inv1",
  inventory_policy: InventoryPolicy.Deny,
  price: 1000,
  taxable: true,
  created_at: "2024-01-01T00:00:00Z",
  updated_at: "2024-01-01T00:00:00Z",
}

const mockProduct = {
  id: "1",
  name: "テスト商品",
  description: "テスト商品の説明",
  status: ProductStatus.Active,
  media: [mockMedia],
  variants: [mockVariant],
}

// モックナビゲーション
const notFoundMock = jest.fn()
jest.mock("next/navigation", () => ({
  ...jest.requireActual("next/navigation"),
  useParams: () => ({ id: "1" }),
  notFound: () => {
    notFoundMock()
    return null
  },
}))

const renderWithClient = (ui: React.ReactElement) => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        throwOnError: false,
      },
    },
  })

  return render(<QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>)
}

describe("ProductPage", () => {
  beforeEach(() => {
    jest.clearAllMocks()
    localStorage.clear()
  })

  it("商品詳細が正しく表示される", async () => {
    server.use(
      getGetProductMockHandler(() => ({
        product: mockProduct,
      })),
    )

    renderWithClient(<ProductPage />)

    // ローディング状態が消えるのを待つ
    await waitForElementToBeRemoved(() => screen.queryByRole("status"), { timeout: 10000 })

    // 商品情報が表示されるのを確認
    expect(screen.getByText("テスト商品")).toBeInTheDocument()
    expect(screen.getByText("テスト商品の説明")).toBeInTheDocument()
    expect(screen.getByText("¥1,000")).toBeInTheDocument()
  })

  it("サイズ選択が機能する", async () => {
    server.use(
      getGetProductMockHandler(() => ({
        product: mockProduct,
      })),
    )

    renderWithClient(<ProductPage />)

    await waitForElementToBeRemoved(() => screen.queryByRole("status"), { timeout: 10000 })

    const sizeL = screen.getByText("L")
    fireEvent.click(sizeL)

    const radioL = screen.getByRole("radio", { name: "L" })
    expect(radioL).toBeChecked()
  })

  it("カラー選択が機能する", async () => {
    server.use(
      getGetProductMockHandler(() => ({
        product: mockProduct,
      })),
    )

    renderWithClient(<ProductPage />)

    await waitForElementToBeRemoved(() => screen.queryByRole("status"), { timeout: 10000 })

    const colorTrigger = screen.getByRole("combobox", { name: /カラー/i })
    fireEvent.click(colorTrigger)

    const navyOption = await screen.findByText("ネイビー")
    fireEvent.click(navyOption)

    await waitFor(() => {
      expect(colorTrigger).toHaveTextContent("ネイビー")
    })
  })

  it("数量選択が機能する", async () => {
    server.use(
      getGetProductMockHandler(() => ({
        product: mockProduct,
      })),
    )

    renderWithClient(<ProductPage />)

    await waitForElementToBeRemoved(() => screen.queryByRole("status"), { timeout: 10000 })

    const quantityTrigger = screen.getByRole("combobox", { name: /数量/i })
    fireEvent.click(quantityTrigger)

    const option3 = await screen.findByRole("option", { name: "3" })
    fireEvent.click(option3)

    await waitFor(() => {
      expect(quantityTrigger).toHaveTextContent("3")
    })
  })

  it("ローディング状態が表示される", async () => {
    server.use(
      getGetProductMockHandler(async () => {
        await new Promise((resolve) => setTimeout(resolve, 100))
        return { product: mockProduct }
      }),
    )

    renderWithClient(<ProductPage />)
    expect(screen.getByRole("status")).toBeInTheDocument()
  })

  it("404エラーの場合、notFound()が呼ばれる", async () => {
    server.use(
      http.get("*/ec-extension/products/:id", async () => {
        await delay(1000)
        return new HttpResponse(JSON.stringify({ message: "Product not found" }), {
          status: 404,
          headers: { "Content-Type": "application/json" },
        })
      }),
    )

    renderWithClient(<ProductPage />)

    await waitFor(
      () => {
        expect(notFoundMock).toHaveBeenCalled()
      },
      { timeout: 10000 },
    )
  })

  it("その他のエラーの場合、エラーページが表示される", async () => {
    const testError = new Error("Request failed with status code 500")
    server.use(
      getGetProductMockHandler(() => {
        throw testError
      }),
    )

    renderWithClient(<ProductPage />)

    await waitFor(
      () => {
        expect(screen.getByText(testError.message)).toBeInTheDocument()
      },
      { timeout: 10000 },
    )
  })
})
