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

// Mock scrollIntoView in JSDOM
window.HTMLElement.prototype.scrollIntoView = jest.fn()

// Mock product data
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

// Mock navigation
const notFoundMock = jest.fn()
jest.mock("next/navigation", () => ({
  ...jest.requireActual("next/navigation"),
  useParams: () => ({ id: "1" }),
  notFound: () => {
    notFoundMock()
    return null
  },
}))

// Common query client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
      throwOnError: false,
    },
  },
})

const renderWithClient = (ui: React.ReactElement) => {
  return render(<QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>)
}

describe("ProductPage", () => {
  beforeAll(() => {
    server.listen({ onUnhandledRequest: "bypass" })
  })

  afterAll(() => {
    server.close()
  })

  beforeEach(() => {
    jest.clearAllMocks()
    localStorage.clear()
    server.resetHandlers()
    queryClient.clear()
  })

  it("displays product details correctly", async () => {
    server.use(
      getGetProductMockHandler(() => ({
        product: mockProduct,
      })),
    )

    renderWithClient(<ProductPage />)

    // Wait for loading state to disappear
    await waitForElementToBeRemoved(() => screen.queryByRole("status"), { timeout: 3000 })

    // Check that product information is displayed
    expect(screen.getByText("テスト商品")).toBeInTheDocument()
    expect(screen.getByText("テスト商品の説明")).toBeInTheDocument()
    expect(screen.getByText("¥1,000")).toBeInTheDocument()
  })

  it("size selection works", async () => {
    server.use(
      getGetProductMockHandler(() => ({
        product: mockProduct,
      })),
    )

    renderWithClient(<ProductPage />)

    await waitForElementToBeRemoved(() => screen.queryByRole("status"), { timeout: 3000 })

    const sizeL = screen.getByRole("radio", { name: "L" })
    fireEvent.click(sizeL)

    expect(sizeL).toBeChecked()
  })

  it("color selection works", async () => {
    server.use(
      getGetProductMockHandler(() => ({
        product: mockProduct,
      })),
    )

    renderWithClient(<ProductPage />)

    await waitForElementToBeRemoved(() => screen.queryByRole("status"), { timeout: 3000 })

    const colorTrigger = screen.getByRole("combobox", { name: /カラー/i })
    fireEvent.click(colorTrigger)

    const navyOption = await screen.findByRole("option", { name: "ネイビー" })
    fireEvent.click(navyOption)

    await waitFor(
      () => {
        expect(colorTrigger).toHaveTextContent("ネイビー")
      },
      { timeout: 1000 },
    )
  })

  it("quantity selection works", async () => {
    server.use(
      getGetProductMockHandler(() => ({
        product: mockProduct,
      })),
    )

    renderWithClient(<ProductPage />)

    await waitForElementToBeRemoved(() => screen.queryByRole("status"), { timeout: 3000 })

    const quantityTrigger = screen.getByRole("combobox", { name: /数量/i })
    fireEvent.click(quantityTrigger)

    const option3 = await screen.findByRole("option", { name: "3" })
    fireEvent.click(option3)

    await waitFor(
      () => {
        expect(quantityTrigger).toHaveTextContent("3")
      },
      { timeout: 1000 },
    )
  })

  it("displays loading state", async () => {
    server.use(
      getGetProductMockHandler(async () => {
        await new Promise((resolve) => setTimeout(resolve, 100))
        return { product: mockProduct }
      }),
    )

    renderWithClient(<ProductPage />)
    expect(screen.getByRole("status")).toBeInTheDocument()
  })

  it("notFound() is called for 404 errors", async () => {
    server.use(
      http.get("*/ec-extension/products/:id", async () => {
        await delay(100)
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
      { timeout: 2000 },
    )
  })

  it("displays error page for other errors", async () => {
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
      { timeout: 2000 },
    )
  })
})
