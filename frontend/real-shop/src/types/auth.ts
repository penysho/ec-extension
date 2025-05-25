export const USER_ID_COOKIE_NAME = "USER_ID"

// Authentication-related type definitions

export type SignUpParameters = {
  email: string
  password: string
}

export type SignInInput = {
  email: string
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
