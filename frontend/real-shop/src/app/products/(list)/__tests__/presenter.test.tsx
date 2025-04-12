import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import "@testing-library/jest-dom"
import { fireEvent, render, screen, waitFor } from "@testing-library/react"

import { InventoryPolicy, MediaStatus, ProductStatus, getGetProductsMockHandler } from "@/generated/backend"
import { server } from "@/mocks/server"

import ProductListPresenter from "../presenter"

const mockProducts = [
  {
    id: "1",
    name: "テスト商品1",
    description: "テスト商品1の説明",
    status: ProductStatus.Active,
    media: [
      {
        id: "media1",
        status: MediaStatus.Active,
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
      },
    ],
    variants: [
      {
        id: "variant1",
        available_for_sale: true,
        list_order: 1,
        inventory_item_id: "inv1",
        inventory_policy: InventoryPolicy.Deny,
        price: 1000,
        taxable: true,
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
      },
    ],
  },
  {
    id: "2",
    name: "テスト商品2",
    description: "テスト商品2の説明",
    status: ProductStatus.Active,
    media: [
      {
        id: "media2",
        status: MediaStatus.Active,
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
      },
    ],
    variants: [
      {
        id: "variant2",
        available_for_sale: true,
        list_order: 2,
        inventory_item_id: "inv2",
        inventory_policy: InventoryPolicy.Deny,
        price: 2000,
        taxable: true,
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
      },
    ],
  },
]

const renderWithClient = (ui: React.ReactElement) => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  })

  return render(<QueryClientProvider client={queryClient}>{ui}</QueryClientProvider>)
}

describe("ProductListPresenter", () => {
  it("商品一覧が正しく表示される", async () => {
    server.use(
      getGetProductsMockHandler(() => ({
        products: mockProducts,
      })),
    )

    renderWithClient(<ProductListPresenter />)

    await waitFor(
      () => {
        expect(screen.getByText("テスト商品1")).toBeInTheDocument()
        expect(screen.getByText("テスト商品2")).toBeInTheDocument()
      },
      { timeout: 5000 },
    )
  })

  it("商品検索が機能する", async () => {
    server.use(
      getGetProductsMockHandler(() => ({
        products: mockProducts,
      })),
    )

    renderWithClient(<ProductListPresenter />)

    await waitFor(
      () => {
        expect(screen.getByText("テスト商品1")).toBeInTheDocument()
      },
      { timeout: 5000 },
    )

    const searchInput = screen.getByPlaceholderText("商品を検索...")
    fireEvent.change(searchInput, { target: { value: "テスト商品1" } })

    await waitFor(
      () => {
        expect(screen.getByText("テスト商品1")).toBeInTheDocument()
        expect(screen.queryByText("テスト商品2")).not.toBeInTheDocument()
      },
      { timeout: 5000 },
    )
  }, 10000)

  it("カテゴリーフィルターが機能する", async () => {
    server.use(
      getGetProductsMockHandler(() => ({
        products: mockProducts,
      })),
    )

    renderWithClient(<ProductListPresenter />)

    await waitFor(
      () => {
        expect(screen.getByText("テスト商品1")).toBeInTheDocument()
      },
      { timeout: 5000 },
    )

    const categoryButton = screen.getByRole("checkbox", { name: "トップス" })
    fireEvent.click(categoryButton)

    await waitFor(
      () => {
        expect(screen.getByText("テスト商品1")).toBeInTheDocument()
      },
      { timeout: 5000 },
    )
  })

  it("ページネーションが機能する", async () => {
    const manyProducts = Array.from({ length: 15 }, (_, i) => ({
      id: String(i + 1),
      name: `テスト商品${i + 1}`,
      description: `テスト商品${i + 1}の説明`,
      status: ProductStatus.Active,
      media: [
        {
          id: `media${i + 1}`,
          status: MediaStatus.Active,
          created_at: "2024-01-01T00:00:00Z",
          updated_at: "2024-01-01T00:00:00Z",
        },
      ],
      variants: [
        {
          id: `variant${i + 1}`,
          available_for_sale: true,
          list_order: i + 1,
          inventory_item_id: `inv${i + 1}`,
          inventory_policy: InventoryPolicy.Deny,
          price: 1000 * (i + 1),
          taxable: true,
          created_at: "2024-01-01T00:00:00Z",
          updated_at: "2024-01-01T00:00:00Z",
        },
      ],
    }))

    server.use(
      getGetProductsMockHandler(() => ({
        products: manyProducts,
      })),
    )

    renderWithClient(<ProductListPresenter />)

    await waitFor(
      () => {
        expect(screen.getByText("テスト商品1")).toBeInTheDocument()
      },
      { timeout: 5000 },
    )

    const nextPageButton = screen.getByRole("button", { name: "2" })
    fireEvent.click(nextPageButton)

    await waitFor(
      () => {
        expect(screen.getByText("テスト商品13")).toBeInTheDocument()
      },
      { timeout: 5000 },
    )
  })

  it("ローディング状態が表示される", async () => {
    server.use(
      getGetProductsMockHandler(async () => {
        await new Promise((resolve) => setTimeout(resolve, 2000))
        return { products: [] }
      }),
    )

    renderWithClient(<ProductListPresenter />)
    expect(screen.getByLabelText("読み込み中")).toBeInTheDocument()
  })

  it("エラー状態が表示される", async () => {
    const testError = new Error("Request failed with status code 500")
    server.use(
      getGetProductsMockHandler(() => {
        throw testError
      }),
    )

    renderWithClient(<ProductListPresenter />)

    await waitFor(
      () => {
        expect(screen.getByText(testError.message)).toBeInTheDocument()
      },
      { timeout: 10000 },
    )
  })
})
