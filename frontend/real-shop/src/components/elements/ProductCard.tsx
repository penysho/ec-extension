import { Heart } from "lucide-react"
import Image from "next/image"
import Link from "next/link"

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter } from "@/components/ui/card"

interface ProductCardProps {
  id: string
  name: string
  price: number
  image: string
  category: string
}

export function ProductCard({
  id,
  name,
  price,
  image,
  category,
}: ProductCardProps) {
  return (
    <Card className="overflow-hidden">
      <Link href={`/products/${id}`}>
        <CardContent className="p-0">
          <div className="relative aspect-square">
            <Image
              src={image}
              alt={name}
              fill
              className="object-cover transition-all duration-300 hover:scale-105"
            />
            <Button
              variant="ghost"
              size="icon"
              className="absolute top-2 right-2 bg-white/80 hover:bg-white"
              aria-label="お気に入りに追加"
            >
              <Heart className="h-5 w-5" />
            </Button>
          </div>
        </CardContent>
        <CardFooter className="flex flex-col items-start p-4">
          <div className="text-sm text-muted-foreground">{category}</div>
          <h3 className="font-semibold text-lg mt-1">{name}</h3>
          <div className="mt-2 font-bold">¥{price.toLocaleString()}</div>
        </CardFooter>
      </Link>
    </Card>
  )
}
