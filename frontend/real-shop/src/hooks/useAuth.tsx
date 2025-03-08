"use client"

import { Amplify } from "aws-amplify"
import { getCurrentUser, signIn, signOut, signUp } from "aws-amplify/auth"
import { cognitoUserPoolsTokenProvider } from "aws-amplify/auth/cognito"
import { KeyValueStorageInterface } from "aws-amplify/utils"
import { useAtom } from "jotai"
import { useCallback, useEffect, useRef } from "react"

import config from "@/amplifyconfiguration.json"
import { usePostSignIn } from "@/generated/backend"
import { errorAtom, loadingAtom, userAtom } from "@/lib/stores"

const temporaryStorage = new (class TemporaryStorage implements KeyValueStorageInterface {
  storageObject: Record<string, string> = {}
  async setItem(key: string, value: string): Promise<void> {
    this.storageObject[key] = value
  }
  async getItem(key: string): Promise<string | null> {
    return this.storageObject[key] || null
  }
  async removeItem(key: string): Promise<void> {
    delete this.storageObject[key]
  }
  async clear(): Promise<void> {
    this.storageObject = {}
  }
})()

Amplify.configure(config)
cognitoUserPoolsTokenProvider.setKeyValueStorage(temporaryStorage)

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
  email: string
  userId: string
} | null

export const useAuth = () => {
  const [user, setUser] = useAtom(userAtom)
  const [loading, setLoading] = useAtom(loadingAtom)
  const [error, setError] = useAtom(errorAtom)

  const { mutate: postSignIn } = usePostSignIn()
  const storageRef = useRef(temporaryStorage)

  // Fetch current authenticated user on initial load
  useEffect(() => {
    const fetchUser = async () => {
      setLoading(true)
      try {
        const authenticatedUser = await getCurrentUser()
        setUser({
          username: authenticatedUser.username,
          email: authenticatedUser.signInDetails?.loginId || "",
          userId: authenticatedUser.userId,
        })
      } catch {
        setUser(null) // User is not authenticated
      } finally {
        setLoading(false)
      }
    }
    fetchUser()
  }, [setUser, setLoading])

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
    [setError, setLoading],
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
            email: authenticatedUser.signInDetails?.loginId || "",
            userId: authenticatedUser.userId,
          })

          const keys = Object.keys(storageRef.current.storageObject)

          let idToken = ""
          let refreshToken = ""
          for (const key of keys) {
            if (key.endsWith("idToken")) {
              idToken = storageRef.current.storageObject[key]
            } else if (key.endsWith("refreshToken")) {
              refreshToken = storageRef.current.storageObject[key]
            }
          }

          postSignIn({
            data: {
              id_token: idToken,
              refresh_token: refreshToken,
            },
          })
        }

        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (err: any) {
        setError(err.message || "Error signing in")
        throw err
      } finally {
        setLoading(false)
      }
    },
    [setError, setLoading, setUser, postSignIn],
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
  }, [setError, setLoading, setUser])

  return {
    user,
    loading,
    error,
    handleSignUp,
    handleSignIn,
    handleSignOut,
  }
}
