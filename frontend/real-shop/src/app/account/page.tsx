"use client"

import { FavoriteItems, OrderHistory, UserProfile } from "@/components/layout/account"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useGetCustomers } from "@/generated/backend"
import { useAuth } from "@/hooks/useAuth"

import Loading from "../loading"

export default function Page() {
  const { user, loading: authLoading, handleSignOut } = useAuth()
  const { error, data, isFetching } = useGetCustomers({ email: user?.email })

  if (authLoading || isFetching) {
    return <Loading />
  }

  if (!user) {
    throw new Error("ログインが必要です")
  }

  if (error) {
    throw new Error("ユーザー情報の取得に失敗しました")
  }

  const customer = data?.customers?.[0]
  if (!customer) {
    throw new Error("ユーザーデータが見つかりませんでした")
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="mb-6 text-3xl font-bold">マイページ</h1>
      <Tabs defaultValue="profile" className="space-y-4">
        <TabsList>
          <TabsTrigger value="profile">プロフィール</TabsTrigger>
          <TabsTrigger value="orders">注文履歴</TabsTrigger>
          <TabsTrigger value="favorites">お気に入り</TabsTrigger>
        </TabsList>
        <TabsContent value="profile">
          <UserProfile user={customer} handleSignOut={handleSignOut} />
        </TabsContent>
        <TabsContent value="orders">
          <OrderHistory userId={user.userId} />
        </TabsContent>
        <TabsContent value="favorites">
          <FavoriteItems userId={user.userId} />
        </TabsContent>
      </Tabs>
    </div>
  )
}
