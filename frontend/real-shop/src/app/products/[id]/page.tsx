"use client"

import { notFound, useParams } from "next/navigation"

import ProductImage from "@/components/elements/ProductImage"
import { useGetProduct } from "@/generated/backend"

import Loading from "./loading"

export default function Page() {
  const params = useParams()

  const id = Number(params.id) || 0

  const { isFetching, error, data } = useGetProduct(id)
  const product = data?.product

  if (isFetching) {
    return <Loading />
  }

  if (error?.status === 404) {
    notFound()
  }

  if (!product || !!error) {
    return error
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
