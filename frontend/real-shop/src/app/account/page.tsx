"use client"

import {
  FavoriteItems,
  OrderHistory,
  UserProfile,
} from "@/components/layouts/account"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useGetCustomers } from "@/generated/backend"
import { useAuth } from "@/hooks/useAuth"

import Loading from "../loading"

export default function AccountPage() {
  const { user, loading, handleSignOut } = useAuth()
  const { isFetching, error, data } = useGetCustomers({ email: user?.email })

  if (isFetching || loading) return <Loading />

  if (!user) {
    return <div>ログインが必要です。</div>
  }
  if (!data || !!error) {
    throw error
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-6">マイページ</h1>
      <Tabs defaultValue="profile" className="space-y-4">
        <TabsList>
          <TabsTrigger value="profile">プロフィール</TabsTrigger>
          <TabsTrigger value="orders">注文履歴</TabsTrigger>
          <TabsTrigger value="favorites">お気に入り</TabsTrigger>
        </TabsList>
        <TabsContent value="profile">
          <UserProfile user={data.customers[0]} handleSignOut={handleSignOut} />
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
