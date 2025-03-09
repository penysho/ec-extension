"use client"

import { zodResolver } from "@hookform/resolvers/zod"
import { confirmSignUp } from "aws-amplify/auth"
import { useAtom } from "jotai"
import { useRouter } from "next/navigation"
import { useState } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"

import { Button } from "@/components/ui/button"
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { pendingVerificationEmailAtom } from "@/lib/stores"

// フォームバリデーションスキーマ
const formSchema = z.object({
  code: z.string().min(6, {
    message: "確認コードは6文字以上で入力してください",
  }),
})

type VerificationFormValues = z.infer<typeof formSchema>

export default function ConfirmPage() {
  const router = useRouter()
  const [email, setEmail] = useAtom(pendingVerificationEmailAtom)
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const form = useForm<VerificationFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      code: "",
    },
  })

  const onSubmit = async (values: VerificationFormValues) => {
    if (!email) {
      setError("メールアドレスが見つかりません。登録画面に戻ってください。")
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      const { isSignUpComplete } = await confirmSignUp({
        username: email,
        confirmationCode: values.code,
      })

      if (isSignUpComplete) {
        setEmail(null)
        sessionStorage.setItem("registration_verified", "true")
        router.push("/registration-complete")
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "確認に失敗しました。もう一度お試しください。")
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className="container mx-auto max-w-md py-12">
      <div className="space-y-6">
        <div className="space-y-2 text-center">
          <h1 className="text-3xl font-bold">アカウント確認</h1>
          {email && (
            <p className="text-gray-500">
              <span className="font-medium">{email}</span> に確認コードを送信しました。 下記に入力してください。
            </p>
          )}
        </div>

        {error && <div className="rounded border border-red-200 bg-red-50 px-4 py-3 text-red-700">{error}</div>}

        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="code"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>確認コード</FormLabel>
                  <FormControl>
                    <Input placeholder="123456" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <Button type="submit" className="w-full" disabled={isSubmitting || !email}>
              {isSubmitting ? "確認中..." : "確認する"}
            </Button>
          </form>
        </Form>

        <div className="text-center">
          <p className="text-sm text-gray-500">
            コードが届きませんか？{" "}
            <button
              type="button"
              className="text-blue-600 hover:underline"
              onClick={() => {
                alert("コードを再送信する機能はここに実装されます")
              }}
            >
              コードを再送信
            </button>
          </p>
        </div>
      </div>
    </div>
  )
}
