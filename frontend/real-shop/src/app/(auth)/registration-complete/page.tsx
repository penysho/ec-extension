"use client"

import { useRouter } from "next/navigation"
import { useEffect, useState } from "react"

import { Button } from "@/components/ui/button"

export default function RegistrationCompletePage() {
  const router = useRouter()
  const [isVerified, setIsVerified] = useState(false)

  useEffect(() => {
    const verified = sessionStorage.getItem("registration_verified")
    if (!verified) {
      router.push("/register")
    } else {
      setIsVerified(true)
    }
  }, [router])

  const handleLoginClick = () => {
    sessionStorage.removeItem("registration_verified")
    router.push("/login")
  }

  if (!isVerified) {
    return null
  }

  return (
    <div className="container mx-auto max-w-md py-12">
      <div className="space-y-8">
        <div className="space-y-2 text-center">
          <div className="flex justify-center">
            <div className="rounded-full bg-green-100 p-3">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-12 w-12 text-green-600"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
            </div>
          </div>
          <h1 className="text-3xl font-bold">登録完了</h1>
          <p className="text-gray-500">アカウントの登録が完了しました。ログインして、サービスをご利用ください。</p>
        </div>

        <div className="space-y-4">
          <Button className="w-full" onClick={handleLoginClick}>
            ログインページへ
          </Button>

          <div className="text-center">
            <p className="text-sm text-gray-500">
              何か問題がありましたか？{" "}
              <a href="/contact" className="text-blue-600 hover:underline">
                サポートに連絡する
              </a>
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
