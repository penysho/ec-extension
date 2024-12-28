"use client"

import { AlertTriangle, Home, RefreshCcw } from "lucide-react"
import Link from "next/link"
import { useEffect } from "react"

import { Button } from "@/components/ui/button"

export default function ErrorPage({
  error,
  reset,
}: {
  error: Error & { digest?: string }
  reset: () => void
}) {
  useEffect(() => {
    // Log the error to an error reporting service
    console.error(error)
  }, [error])

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center justify-center p-4">
      <div className="max-w-md w-full bg-white shadow-lg rounded-lg p-8 text-center">
        <AlertTriangle className="h-16 w-16 text-red-500 mx-auto mb-6 animate-pulse" />
        <h1 className="text-2xl font-bold text-gray-900 mb-2">
          エラーが発生しました
        </h1>
        <p className="text-gray-600 mb-6">
          {error.message ||
            "予期せぬエラーが発生しました。ご不便をおかけして申し訳ございません。"}
        </p>
        <div className="space-y-4">
          <Button
            onClick={() => reset()}
            className="w-full flex items-center justify-center"
          >
            <RefreshCcw className="mr-2 h-4 w-4" />
            もう一度試す
          </Button>
          <Button asChild variant="outline" className="w-full">
            <Link href="/">
              <Home className="mr-2 h-4 w-4" />
              ホームページに戻る
            </Link>
          </Button>
        </div>
        {error.digest && (
          <p className="mt-6 text-xs text-gray-500">
            エラーコード: {error.digest}
          </p>
        )}
      </div>
    </div>
  )
}
