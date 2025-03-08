"use client"
import Image from "next/image"
import Link from "next/link"

import { ProductCard } from "@/components/layout/top"
import { Button } from "@/components/ui/button"

// 仮のデータ
const newProducts = [
  {
    id: "1",
    name: "スタイリッシュTシャツ",
    price: 2980,
    image: "/images/tshirt.jpg",
    category: "トップス",
    isNew: true,
  },
  {
    id: "2",
    name: "デニムジーンズ",
    price: 7980,
    image: "/images/jeans.jpg",
    category: "ボトムス",
  },
  {
    id: "3",
    name: "レザージャケット",
    price: 29800,
    image: "/images/jacket.jpg",
    category: "アウター",
    isSale: true,
  },
  {
    id: "4",
    name: "スニーカー",
    price: 8980,
    image: "/images/sneakers.jpg",
    category: "シューズ",
  },
]

const categories = [
  { name: "トップス", image: "https://placehold.jp/600x600.png" },
  { name: "ボトムス", image: "https://placehold.jp/600x600.png" },
  { name: "アウター", image: "https://placehold.jp/600x600.png" },
  { name: "シューズ", image: "https://placehold.jp/600x600.png" },
]

export default function Home() {
  return (
    <div className="container mx-auto px-4 py-8">
      {/* ヒーローセクション */}
      <section className="relative mb-12 h-[70vh]">
        <Image src="https://placehold.jp/1000x1000.png" alt="新作コレクション" fill className="object-cover" />
        <div className="absolute inset-0 flex flex-col items-center justify-center bg-black bg-opacity-40 text-white">
          <h1 className="mb-4 text-4xl font-bold md:text-6xl">新作コレクション</h1>
          <p className="mb-8 text-xl">最新のトレンドをチェック</p>
          <Button size="lg" asChild>
            <Link href="/products">今すぐ購入</Link>
          </Button>
        </div>
      </section>

      {/* 人気カテゴリー */}
      <section className="mb-12">
        <h2 className="mb-6 text-3xl font-bold">人気カテゴリー</h2>
        <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
          {categories.map((category) => (
            <Link
              key={category.name}
              href={`/category/${category.name}`}
              className="group relative aspect-square overflow-hidden"
            >
              <Image
                src={category.image}
                alt={category.name}
                fill
                className="object-cover transition-transform duration-300 group-hover:scale-110"
              />
              <div className="absolute inset-0 flex items-center justify-center bg-black bg-opacity-30">
                <span className="text-xl font-semibold text-white">{category.name}</span>
              </div>
            </Link>
          ))}
        </div>
      </section>

      {/* 新着商品 */}
      <section className="mb-12">
        <h2 className="mb-6 text-3xl font-bold">新着商品</h2>
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-4">
          {newProducts.map((product) => (
            <ProductCard key={product.id} {...product} />
          ))}
        </div>
      </section>

      {/* セール情報 */}
      <section className="mb-12 rounded-lg bg-red-600 px-6 py-12 text-white">
        <div className="text-center">
          <h2 className="mb-4 text-3xl font-bold">サマーセール開催中！</h2>
          <p className="mb-6 text-xl">全商品最大50%OFF</p>
          <Button size="lg" variant="secondary" asChild>
            <Link href="/sale">セール商品をチェック</Link>
          </Button>
        </div>
      </section>

      {/* ブランドストーリー */}
      <section className="mb-12 flex flex-col items-center gap-8 md:flex-row">
        <div className="md:w-1/2">
          <h2 className="mb-4 text-3xl font-bold">ブランドストーリー</h2>
          <p className="mb-4">
            私たちは、高品質な素材と職人技にこだわり、環境に配慮した持続可能なファッションを提供しています。
            一つ一つの商品に込められた想いと技術を、あなたの日常に。
          </p>
          <Button variant="outline" asChild>
            <Link href="/about">詳しく見る</Link>
          </Button>
        </div>
        <div className="md:w-1/2">
          <Image
            src="https://placehold.jp/600x600.png"
            alt="ブランドストーリー"
            width={600}
            height={400}
            className="rounded-lg"
          />
        </div>
      </section>
    </div>
  )
}
