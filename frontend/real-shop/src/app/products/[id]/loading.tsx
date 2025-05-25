import { Skeleton } from "@/components/ui/skeleton"

export default function Loading() {
  return (
    <div className="container mx-auto px-4 py-6" role="status" aria-label="読み込み中">
      <div className="grid grid-cols-1 gap-8 lg:grid-cols-2">
        {/* Product image skeleton */}
        <div className="relative aspect-square">
          <Skeleton className="h-full w-full rounded-lg" />
        </div>

        {/* Product information skeleton */}
        <div className="space-y-4">
          <Skeleton className="h-8 w-3/4" /> {/* Product name */}
          <div className="space-y-2">
            <Skeleton className="h-4 w-full" /> {/* Description line 1 */}
            <Skeleton className="h-4 w-5/6" /> {/* Description line 2 */}
            <Skeleton className="h-4 w-4/6" /> {/* Description line 3 */}
          </div>
          <Skeleton className="h-8 w-32" /> {/* Price */}
          <Skeleton className="h-12 w-40" /> {/* Add to cart button */}
        </div>
      </div>
    </div>
  )
}
