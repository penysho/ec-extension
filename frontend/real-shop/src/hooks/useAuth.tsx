"use client"

import { Amplify } from "aws-amplify"
import { getCurrentUser, signIn, signOut, signUp } from "aws-amplify/auth"
import Cookies from "js-cookie"
import { useCallback, useEffect, useState } from "react"

import config from "@/amplifyconfiguration.json"

Amplify.configure(config)
// cognitoUserPoolsTokenProvider.setKeyValueStorage(new CookieStorage())

type SignUpParameters = {
  username: string
  password: string
  email: string
  phone_number: string
}

type SignInInput = {
  username: string
  password: string
}

export type User = {
  username: string
  userId: string
} | null

export const useAuth = () => {
  const [user, setUser] = useState<User>(null)
  // const [user, setUser] = useAtom(userAtom)
  const [loading, setLoading] = useState<boolean>(true)
  const [error, setError] = useState<string | null>(null)

  // Fetch current authenticated user on initial load
  useEffect(() => {
    const fetchUser = async () => {
      setLoading(true)
      try {
        const authenticatedUser = await getCurrentUser()
        setUser({
          username: authenticatedUser.username,
          userId: authenticatedUser.userId,
        })
      } catch {
        setUser(null) // User is not authenticated
      } finally {
        setLoading(false)
      }
    }
    fetchUser()
  }, [])

  // Sign up a new user
  const handleSignUp = useCallback(
    async ({ username, password, email, phone_number }: SignUpParameters) => {
      setError(null)
      setLoading(true)
      try {
        const { userId } = await signUp({
          username,
          password,
          options: {
            userAttributes: {
              email,
              phone_number,
            },
          },
        })
        return userId
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (err: any) {
        setError(err.message || "Error signing up")
        throw err
      } finally {
        setLoading(false)
      }
    },
    [],
  )

  // Sign in an existing user
  const handleSignIn = useCallback(
    async ({ username, password }: SignInInput) => {
      setError(null)
      setLoading(true)
      try {
        const { isSignedIn } = await signIn({ username, password })
        if (isSignedIn) {
          const authenticatedUser = await getCurrentUser()
          setUser({
            username: authenticatedUser.username,
            userId: authenticatedUser.userId,
          })

          Cookies.set("userId", authenticatedUser.userId)
        }
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (err: any) {
        setError(err.message || "Error signing in")
        throw err
      } finally {
        setLoading(false)
      }
    },
    [],
  )

  // Sign out the current user
  const handleSignOut = useCallback(async () => {
    setError(null)
    setLoading(true)
    try {
      await signOut({ global: true })
      setUser(null)
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (err: any) {
      setError(err.message || "Error signing out")
      throw err
    } finally {
      setLoading(false)
    }
  }, [])

  return {
    user,
    loading,
    error,
    handleSignUp,
    handleSignIn,
    handleSignOut,
  }
}
