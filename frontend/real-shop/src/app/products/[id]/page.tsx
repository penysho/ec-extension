"use client"
import { useQuery } from "@tanstack/react-query"
import { useParams, useRouter } from "next/navigation"
import { useEffect } from "react"

import ProductImage from "@/components/elements/productImage"

export default function ProductDetail() {
  const router = useRouter()
  const params = useParams()

  const id = params.id
  const { isLoading, error, data } = useQuery({
    queryKey: [id],
    queryFn: () =>
      fetch(
        `${process.env.NEXT_PUBLIC_BACKEND_ENDPOINT}/ec-extension/products/${id}`,
      ).then((res) => res.json()),
  })

  useEffect(() => {
    if (error) {
      router.push("/error")
    }
  }, [error, router])

  return (
    <div className="container mx-auto px-4 py-6">
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {!isLoading && (
          <>
            <ProductImage url={data.product.media[0].content.image.src} />

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
          </>
        )}
      </div>
    </div>
  )
}
