export const USER_ID_COOKIE_NAME = "USER_ID"

// 認証関連の型定義

export type SignUpParameters = {
  username: string
  password: string
  email: string
  phone_number: string
}

export type SignInInput = {
  username: string
  password: string
}

export type User = {
  username: string
  email: string
  userId: string
} | null

export type AuthTokens = {
  idToken: string
  refreshToken: string
}
