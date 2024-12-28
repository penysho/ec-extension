import { Home, Search, ShoppingBag } from "lucide-react"
import Link from "next/link"

import { Button } from "@/components/ui/button"

export default function NotFound() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 px-4 text-center">
      <ShoppingBag className="h-24 w-24 text-primary mb-8 " />
      <h1 className="text-4xl font-bold text-gray-900 mb-2">
        ページが見つかりません
      </h1>
      <p className="text-xl text-gray-600 mb-8">
        申し訳ありませんが、お探しのページは存在しないようです。
      </p>
      <div className="flex flex-col sm:flex-row gap-4">
        <Button asChild variant="default">
          <Link href="/">
            <Home className="mr-2 h-4 w-4" />
            ホームに戻る
          </Link>
        </Button>
        <Button asChild variant="outline">
          <Link href="/search">
            <Search className="mr-2 h-4 w-4" />
            商品を探す
          </Link>
        </Button>
      </div>
      <p className="mt-8 text-sm text-gray-500">
        エラーコード: 404 | お探しのリソースが見つかりません
      </p>
    </div>
  )
}
