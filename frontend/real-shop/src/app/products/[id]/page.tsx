"use client"
import { useQuery } from "@tanstack/react-query"

import ProductImage from "@/components/elements/productImage"

export default function ProductDetail() {
  const { isPending, error, data } = useQuery({
    queryKey: ["fetchDate"],
    queryFn: () =>
      fetch("http://localhost:8011/ec-extension/products/7853275152557").then(
        (res) => res.json(),
      ),
  })

  if (isPending) return "Loading..."

  if (error) return "An error has occurred: " + error.message

  return (
    <div className="container mx-auto px-4 py-6">
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* 商品画像 */}
        <div className="relative">
          <ProductImage url={data.product.media[0].content.image.src} />
        </div>

        {/* 商品情報 */}
        <div className="space-y-4">
          <h1 className="text-3xl font-bold text-gray-800">
            {data.product.name}
          </h1>
          <p className="text-gray-600">{data.product.description}</p>
          <p className="text-2xl font-semibold text-gray-800">
            ¥{data.product.variants[0].price}
          </p>
          <button className="px-6 py-2 bg-blue-600 text-white rounded-lg shadow hover:bg-blue-700 transition">
            カートに追加
          </button>
        </div>
      </div>
    </div>
  )
}
