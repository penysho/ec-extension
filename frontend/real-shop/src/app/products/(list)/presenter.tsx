"use client"
import { useQueryClient } from "@tanstack/react-query"
import { Search } from "lucide-react"
import { useState } from "react"

import { Pagination } from "@/components/elements/Pagination"
import { ProductCard } from "@/components/elements/ProductCard"
import { Sidebar } from "@/components/elements/Sidebar"
import { Input } from "@/components/ui/input"
import {
  GetProductsResponseResponse,
  Product,
  useGetProducts,
} from "@/generated/backend"

const categories = [
  "トップス",
  "ボトムス",
  "アウター",
  "シューズ",
  "アクセサリー",
]

export default function ProductListPresenter() {
  const [selectedCategories, setSelectedCategories] = useState<string[]>([])
  const [sortOption, setSortOption] = useState("newest")
  const [currentPage, setCurrentPage] = useState(1)
  const [searchQuery, setSearchQuery] = useState("")

  const queryClient = useQueryClient()
  let productsData = queryClient.getQueryData<GetProductsResponseResponse>([
    "products",
  ])

  if (!productsData) {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const { isLoading, error, data } = useGetProducts()
    if (isLoading) return <div>Loading...</div>
    if (error) return <div>Error: {error.message}</div>
    productsData = data?.data
  }

  const products = productsData?.products

  const itemsPerPage = 12

  let totalPages = 1
  let displayedProducts = [] as Product[]
  if (products) {
    totalPages = Math.ceil(products.length / itemsPerPage)
    displayedProducts = products.slice(
      (currentPage - 1) * itemsPerPage,
      currentPage * itemsPerPage,
    )
  }

  const handleCategoryChange = (category: string) => {
    setSelectedCategories((prev) =>
      prev.includes(category)
        ? prev.filter((c) => c !== category)
        : [...prev, category],
    )
    setCurrentPage(1)
  }

  const handleSortChange = (option: string) => {
    setSortOption(option)
    setCurrentPage(1)
  }

  const handlePageChange = (page: number) => {
    setCurrentPage(page)
  }

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value)
    setCurrentPage(1)
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-8">商品一覧</h1>
      <div className="flex mb-6">
        <div className="relative flex-grow">
          <Input
            type="text"
            placeholder="商品を検索..."
            value={searchQuery}
            onChange={handleSearchChange}
            className="pl-10"
          />
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" />
        </div>
      </div>
      <div className="flex">
        <Sidebar
          categories={categories}
          selectedCategories={selectedCategories}
          onCategoryChange={handleCategoryChange}
          sortOption={sortOption}
          onSortChange={handleSortChange}
        />
        <div className="flex-grow ml-8">
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
            {displayedProducts.map((product) => (
              <ProductCard key={product.id} product={product} />
            ))}
          </div>
          <Pagination
            currentPage={currentPage}
            totalPages={totalPages}
            onPageChange={handlePageChange}
          />
        </div>
      </div>
    </div>
  )
}
