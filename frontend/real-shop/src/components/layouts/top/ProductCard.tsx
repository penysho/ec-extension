import Image from "next/image"
import Link from "next/link"

import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardFooter } from "@/components/ui/card"

interface ProductCardProps {
  id: string
  name: string
  price: number
  image: string
  category: string
  isNew?: boolean
  isSale?: boolean
}

export function ProductCard({
  id,
  name,
  price,
  image,
  category,
  isNew,
  isSale,
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
            {isNew && (
              <Badge className="absolute top-2 left-2" variant="secondary">
                NEW
              </Badge>
            )}
            {isSale && (
              <Badge className="absolute top-2 right-2" variant="destructive">
                SALE
              </Badge>
            )}
          </div>
        </CardContent>
      </Link>
      <CardFooter className="flex flex-col items-start p-4">
        <div className="text-sm text-muted-foreground">{category}</div>
        <h3 className="font-semibold text-lg mt-1">{name}</h3>
        <div className="mt-2 font-bold">¥{price.toLocaleString()}</div>
      </CardFooter>
    </Card>
  )
}
