import Image from "next/image"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"

interface FavoriteItem {
  id: string
  name: string
  price: number
  image: string
}

interface FavoriteItemsProps {
  userId: string
}

// Mock data
const mockFavorites: FavoriteItem[] = [
  {
    id: "1",
    name: "スタイリッシュTシャツ",
    price: 2980,
    image: "/images/tshirt.jpg",
  },
  { id: "2", name: "デニムジーンズ", price: 7980, image: "/images/jeans.jpg" },
  {
    id: "3",
    name: "レザージャケット",
    price: 29800,
    image: "/images/jacket.jpg",
  },
]

export function FavoriteItems({ userId }: FavoriteItemsProps) {
  // TODO: implements fetch favorite items
  console.log(userId)
  return (
    <Card>
      <CardHeader>
        <CardTitle>お気に入り商品</CardTitle>
        <CardDescription>あなたがお気に入りに追加した商品です</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
          {mockFavorites.map((item) => (
            <div key={item.id} className="rounded border p-4">
              <div className="relative mb-2 aspect-square">
                <Image src={item.image} alt={item.name} fill className="rounded object-cover" />
              </div>
              <h3 className="font-semibold">{item.name}</h3>
              <p className="text-gray-600">¥{item.price.toLocaleString()}</p>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
