import { useRouter } from "next/navigation"
import { useState } from "react"

import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

interface User {
  id: string
  email?: string | null
  firstName?: string | null
  lastName?: string | null
  phone?: string | null
}

interface UserProfileProps {
  user: User
  handleSignOut: () => Promise<void>
}

export function UserProfile({ user, handleSignOut }: UserProfileProps) {
  const router = useRouter()
  const [isEditing, setIsEditing] = useState(false)
  const [formData, setFormData] = useState<User>(user)

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => {
    const { name, value } = e.target
    setFormData((prev) => ({ ...prev, [name]: value }))
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    // ここでユーザー情報の更新処理を行う
    console.log("Updated user info:", formData)
    setIsEditing(false)
  }

  const handleLogout = async () => {
    try {
      await handleSignOut()
      router.push("/")
    } catch (error) {
      console.error("Logout failed:", error)
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>プロフィール情報</CardTitle>
        <CardDescription>あなたの個人情報を管理します</CardDescription>
      </CardHeader>
      <CardContent>
        {isEditing ? (
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label htmlFor="firstName">名</Label>
                <Input
                  id="firstName"
                  name="firstName"
                  value={formData.firstName ?? ""}
                  onChange={handleInputChange}
                />
              </div>
              <div>
                <Label htmlFor="lastName">姓</Label>
                <Input
                  id="lastName"
                  name="lastName"
                  value={formData.lastName ?? ""}
                  onChange={handleInputChange}
                />
              </div>
            </div>
            <div>
              <Label htmlFor="email">メールアドレス</Label>
              <Input
                id="email"
                name="email"
                type="email"
                value={formData.email ?? ""}
                onChange={handleInputChange}
              />
            </div>

            <div>
              <Label htmlFor="phoneNumber">電話番号</Label>
              <Input
                id="phoneNumber"
                name="phoneNumber"
                value={formData.phone ?? ""}
                onChange={handleInputChange}
              />
            </div>
            <div className="flex space-x-2">
              <Button type="submit">保存</Button>
              <Button
                type="button"
                variant="outline"
                onClick={() => setIsEditing(false)}
              >
                キャンセル
              </Button>
            </div>
          </form>
        ) : (
          <Button onClick={() => setIsEditing(true)}>編集</Button>
        )}
        <div className="mt-6 pt-6 border-t">
          <Button variant="destructive" onClick={handleLogout}>
            ログアウト
          </Button>
        </div>
      </CardContent>
    </Card>
  )
}
