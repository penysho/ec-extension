"use client"
import { Heart, RotateCcw, Share2, Shield, Truck } from "lucide-react"
import { notFound, useParams } from "next/navigation"

import { ProductGallery } from "@/components/layouts/product"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
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
    throw error
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-12">
        {/* 商品画像ギャラリー */}
        <ProductGallery
          images={product.media.map((m) => ({
            src: m.content?.image?.src || "",
            alt: m.content?.image?.alt || product.name,
          }))}
        />

        {/* 商品情報 */}
        <div className="space-y-6">
          <div>
            <h1 className="text-3xl font-bold text-gray-900">{product.name}</h1>
            <p className="mt-2 text-lg font-semibold text-gray-900">
              ¥{product.variants[0]?.price.toLocaleString()}
            </p>
          </div>

          {/* サイズ選択 */}
          <div className="space-y-2">
            <Label>サイズ</Label>
            <RadioGroup defaultValue="M" className="grid grid-cols-5 gap-2">
              {["XS", "S", "M", "L", "XL"].map((size) => (
                <Label
                  key={size}
                  className="flex items-center justify-center p-2 border rounded-md cursor-pointer hover:bg-gray-50 [&:has(:checked)]:border-blue-600 [&:has(:checked)]:bg-blue-50"
                >
                  <RadioGroupItem value={size} className="sr-only" />
                  {size}
                </Label>
              ))}
            </RadioGroup>
          </div>

          {/* カラー選択 */}
          <div className="space-y-2">
            <Label>カラー</Label>
            <Select defaultValue="black">
              <SelectTrigger>
                <SelectValue placeholder="カラーを選択" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="black">ブラック</SelectItem>
                <SelectItem value="white">ホワイト</SelectItem>
                <SelectItem value="navy">ネイビー</SelectItem>
              </SelectContent>
            </Select>
          </div>

          {/* 数量選択 */}
          <div className="space-y-2">
            <Label>数量</Label>
            <Select defaultValue="1">
              <SelectTrigger className="w-24">
                <SelectValue placeholder="数量" />
              </SelectTrigger>
              <SelectContent>
                {[1, 2, 3, 4, 5].map((num) => (
                  <SelectItem key={num} value={num.toString()}>
                    {num}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* アクションボタン */}
          <div className="flex gap-4">
            <Button className="flex-1">カートに追加</Button>
            <Button variant="outline" size="icon">
              <Heart className="h-5 w-5" />
            </Button>
            <Button variant="outline" size="icon">
              <Share2 className="h-5 w-5" />
            </Button>
          </div>

          {/* 配送・保証情報 */}
          <div className="grid grid-cols-2 gap-4 pt-6 border-t">
            <div className="flex items-center gap-2 text-sm text-gray-600">
              <Truck className="h-5 w-5" />
              <span>最短翌日お届け</span>
            </div>
            <div className="flex items-center gap-2 text-sm text-gray-600">
              <Shield className="h-5 w-5" />
              <span>30日間返品保証</span>
            </div>
            <div className="flex items-center gap-2 text-sm text-gray-600">
              <RotateCcw className="h-5 w-5" />
              <span>簡単返品・交換</span>
            </div>
          </div>

          {/* 商品詳細タブ */}
          <Tabs defaultValue="description" className="pt-8">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="description">商品説明</TabsTrigger>
              <TabsTrigger value="details">商品詳細</TabsTrigger>
              <TabsTrigger value="size">サイズ表</TabsTrigger>
            </TabsList>
            <TabsContent value="description" className="mt-4">
              <div className="prose prose-sm max-w-none">
                <p>{product.description}</p>
                {/* <ul className="mt-4">
                  {product.features?.map((feature, index) => (
                    <li key={index}>{feature}</li>
                  ))}
                </ul> */}
              </div>
            </TabsContent>
            <TabsContent value="details" className="mt-4">
              <div className="grid grid-cols-2 gap-x-4 gap-y-2 text-sm">
                <div className="font-medium">素材</div>
                <div>コットン 100%</div>
                <div className="font-medium">原産国</div>
                <div>日本</div>
              </div>
            </TabsContent>
            <TabsContent value="size" className="mt-4">
              <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200">
                  <thead>
                    <tr>
                      <th className="px-4 py-2">サイズ</th>
                      <th className="px-4 py-2">着丈</th>
                      <th className="px-4 py-2">身幅</th>
                      <th className="px-4 py-2">肩幅</th>
                      <th className="px-4 py-2">袖丈</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200">
                    {["XS", "S", "M", "L", "XL"].map((size) => (
                      <tr key={size}>
                        <td className="px-4 py-2 text-center">{size}</td>
                        <td className="px-4 py-2 text-center">62</td>
                        <td className="px-4 py-2 text-center">45</td>
                        <td className="px-4 py-2 text-center">40</td>
                        <td className="px-4 py-2 text-center">22</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </div>
  )
}
