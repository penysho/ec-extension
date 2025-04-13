import { Skeleton } from "@/components/ui/skeleton"

export default function Loading() {
  return (
    <div className="container mx-auto px-4 py-6" role="status" aria-label="読み込み中">
      <div className="grid grid-cols-1 gap-8 lg:grid-cols-2">
        {/* 商品画像のスケルトン */}
        <div className="relative aspect-square">
          <Skeleton className="h-full w-full rounded-lg" />
        </div>

        {/* 商品情報のスケルトン */}
        <div className="space-y-4">
          <Skeleton className="h-8 w-3/4" /> {/* 商品名 */}
          <div className="space-y-2">
            <Skeleton className="h-4 w-full" /> {/* 説明文1行目 */}
            <Skeleton className="h-4 w-5/6" /> {/* 説明文2行目 */}
            <Skeleton className="h-4 w-4/6" /> {/* 説明文3行目 */}
          </div>
          <Skeleton className="h-8 w-32" /> {/* 価格 */}
          <Skeleton className="h-12 w-40" /> {/* カートに追加ボタン */}
        </div>
      </div>
    </div>
  )
}
