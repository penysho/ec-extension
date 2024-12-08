import { Loader2 } from "lucide-react"

export default function Loading() {
  return (
    <div className="flex items-center justify-center min-h-screen">
      <Loader2
        className="w-8 h-8 text-gray-700 animate-spin"
        aria-label="読み込み中"
      />
    </div>
  )
}
