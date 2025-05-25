import { ChevronLeft, ChevronRight } from "lucide-react"
import Image from "next/image"
import Link from "next/link"
import { useRef } from "react"

import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { ProductSummary } from "@/generated/backend"

interface RelatedProductsProps {
  products: ProductSummary[]
}

export function RelatedProducts({ products }: RelatedProductsProps) {
  const scrollRef = useRef<HTMLDivElement>(null)

  const scroll = (direction: "left" | "right") => {
    if (scrollRef.current) {
      const scrollAmount = 320
      scrollRef.current.scrollBy({
        left: direction === "left" ? -scrollAmount : scrollAmount,
        behavior: "smooth",
      })
    }
  }

  if (products.length === 0) {
    return null
  }

  return (
    <div className="mt-16">
      <div className="mb-8 flex items-center justify-between">
        <h2 className="text-2xl font-bold">関連商品</h2>
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="icon"
            onClick={() => scroll("left")}
            className="rounded-full"
            aria-label="前の商品を見る"
          >
            <ChevronLeft className="h-4 w-4" />
          </Button>
          <Button
            variant="outline"
            size="icon"
            onClick={() => scroll("right")}
            className="rounded-full"
            aria-label="次の商品を見る"
          >
            <ChevronRight className="h-4 w-4" />
          </Button>
        </div>
      </div>

      <div
        ref={scrollRef}
        className="scrollbar-hide flex gap-4 overflow-x-auto"
        style={{ scrollbarWidth: "none", msOverflowStyle: "none" }}
      >
        {products.map((product) => (
          <Link key={product.id} href={`/products/${product.id}`} className="min-w-[280px] flex-shrink-0">
            <Card className="overflow-hidden transition-all hover:shadow-lg">
              <CardContent className="p-0">
                <div className="relative aspect-square overflow-hidden bg-gray-100">
                  {product.featured_media_url ? (
                    <Image
                      src={product.featured_media_url}
                      alt={product.name}
                      fill
                      sizes="280px"
                      className="object-cover transition-transform duration-300 hover:scale-105"
                      priority={false}
                    />
                  ) : (
                    <div className="flex h-full items-center justify-center text-gray-400">
                      <svg className="h-16 w-16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                        />
                      </svg>
                    </div>
                  )}
                </div>
                <div className="p-4">
                  <p className="text-xs text-gray-500">{product.vendor}</p>
                  <h3 className="mt-1 line-clamp-2 text-sm font-medium text-gray-900">{product.name}</h3>
                  <p className="mt-2 text-lg font-semibold">¥{product.price.toLocaleString()}</p>
                </div>
              </CardContent>
            </Card>
          </Link>
        ))}
      </div>
    </div>
  )
}
