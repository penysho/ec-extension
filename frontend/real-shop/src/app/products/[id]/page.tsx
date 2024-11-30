"use client"

import { useParams, useRouter } from "next/navigation"
import { useEffect } from "react"

import ProductImage from "@/components/elements/productImage"
import { useGetProduct } from "@/generated/backend"

export default function ProductDetail() {
  const router = useRouter()
  const params = useParams()

  const id = Number(params.id) || 0

  const { isLoading, error, data } = useGetProduct(id)
  const product = data?.data.product

  useEffect(() => {
    if (error) {
      router.push("/error")
    }
  }, [error, router])

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <p className="text-gray-600">Loading...</p>
      </div>
    )
  }

  if (!product) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <p className="text-gray-600">商品が見つかりません。</p>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-6">
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* 商品画像 */}
        <div className="relative">
          <ProductImage url={product.media[0]?.content?.image?.src || ""} />
        </div>

        {/* 商品情報 */}
        <div className="space-y-4">
          <h1 className="text-3xl font-bold text-gray-800">{product.name}</h1>
          <p className="text-gray-600">{product.description}</p>
          <p className="text-2xl font-semibold text-gray-800">
            ¥{product.variants[0]?.price}
          </p>
          <button className="px-6 py-2 bg-blue-600 text-white rounded-lg shadow hover:bg-blue-700 transition">
            カートに追加
          </button>
        </div>
      </div>
    </div>
  )
}
