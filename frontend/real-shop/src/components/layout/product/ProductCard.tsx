import { Heart } from "lucide-react"
import { default as Image } from "next/image"
import Link from "next/link"

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter } from "@/components/ui/card"
import { Product } from "@/generated/backend"

interface ProductCardProps {
  product: Product
}

export function ProductCard({ product }: ProductCardProps) {
  return (
    <Card className="overflow-hidden">
      <Link href={`/products/${product.id}`}>
        <CardContent className="p-0">
          <div className="relative aspect-square">
            <Image
              src={product.media[0].content?.image?.src ? product.media[0].content?.image?.src : "/no-image.png"}
              alt={product.name}
              fill
              className="object-cover transition-all duration-300 hover:scale-105"
            />
            <Button
              variant="ghost"
              size="icon"
              className="absolute right-2 top-2 bg-white/80 hover:bg-white"
              aria-label="お気に入りに追加"
            >
              <Heart className="h-5 w-5" />
            </Button>
          </div>
        </CardContent>
        <CardFooter className="flex flex-col items-start p-4">
          <div className="text-sm text-muted-foreground">dummy</div>
          <h3 className="mt-1 text-lg font-semibold">{product.name}</h3>
          <div className="mt-2 font-bold">¥{product.variants[0].price.toLocaleString()}</div>
        </CardFooter>
      </Link>
    </Card>
  )
}
