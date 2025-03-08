"use client"
import { Search } from "lucide-react"
import { useState } from "react"

import ErrorPage from "@/app/error"
import Loading from "@/app/loading"
import { Pagination, ProductCard, Sidebar } from "@/components/layout/product"
import { Input } from "@/components/ui/input"
import { Product, useGetProducts } from "@/generated/backend"

const categories = ["トップス", "ボトムス", "アウター", "シューズ", "アクセサリー"]

export default function ProductListPresenter() {
  const [selectedCategories, setSelectedCategories] = useState<string[]>([])
  const [sortOption, setSortOption] = useState("newest")
  const [currentPage, setCurrentPage] = useState(1)
  const [searchQuery, setSearchQuery] = useState("")

  const { isFetching, error, data } = useGetProducts()

  if (isFetching) return <Loading />
  if (error) return <ErrorPage error={error} reset={() => {}} />

  const products = data?.products

  const itemsPerPage = 12

  let totalPages = 1
  let displayedProducts = [] as Product[]
  if (products) {
    totalPages = Math.ceil(products.length / itemsPerPage)
    displayedProducts = products.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
  }

  const handleCategoryChange = (category: string) => {
    setSelectedCategories((prev) =>
      prev.includes(category) ? prev.filter((c) => c !== category) : [...prev, category],
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
      <h1 className="mb-8 text-3xl font-bold">商品一覧</h1>
      <div className="mb-6 flex">
        <div className="relative flex-grow">
          <Input
            type="text"
            placeholder="商品を検索..."
            value={searchQuery}
            onChange={handleSearchChange}
            className="pl-10"
          />
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 transform text-gray-400" />
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
        <div className="ml-8 flex-grow">
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
            {displayedProducts.map((product) => (
              <ProductCard key={product.id} product={product} />
            ))}
          </div>
          <Pagination currentPage={currentPage} totalPages={totalPages} onPageChange={handlePageChange} />
        </div>
      </div>
    </div>
  )
}
