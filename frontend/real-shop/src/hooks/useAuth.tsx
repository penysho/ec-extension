"use client"

import { Amplify } from "aws-amplify"
import { getCurrentUser, signIn, signOut, signUp } from "aws-amplify/auth"
import { cognitoUserPoolsTokenProvider } from "aws-amplify/auth/cognito"
import { KeyValueStorageInterface } from "aws-amplify/utils"
import { useAtom } from "jotai"
import { useCallback, useEffect, useRef } from "react"

import { usePostSignIn } from "@/generated/backend"
import { errorAtom, loadingAtom, userAtom } from "@/lib/stores"
import { AuthTokens, SignInInput, SignUpParameters, User } from "@/types/auth"

// In-memory storage implementation
class MemoryStorage implements KeyValueStorageInterface {
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

  // Helper method to retrieve tokens
  getTokens(): AuthTokens {
    const keys = Object.keys(this.storageObject)
    let idToken = ""
    let refreshToken = ""

    for (const key of keys) {
      if (key.endsWith("idToken")) {
        idToken = this.storageObject[key]
      } else if (key.endsWith("refreshToken")) {
        refreshToken = this.storageObject[key]
      }
    }

    return { idToken, refreshToken }
  }
}

// Create singleton instance
const memoryStorage = new MemoryStorage()

// Amplify configuration
Amplify.configure({
  Auth: {
    Cognito: {
      userPoolId: process.env.NEXT_PUBLIC_AUTH_USER_POOL_ID || "",
      userPoolClientId: process.env.NEXT_PUBLIC_AUTH_USER_POOL_CLIENT_ID || "",
    },
  },
})
cognitoUserPoolsTokenProvider.setKeyValueStorage(memoryStorage)

/**
 * Provides authentication hook
 * @returns Authentication state and operation methods
 */
export const useAuth = () => {
  const [user, setUser] = useAtom(userAtom)
  const [loading, setLoading] = useAtom(loadingAtom)
  const [error, setError] = useAtom(errorAtom)

  const { mutateAsync: postSignIn } = usePostSignIn()
  const storageRef = useRef(memoryStorage)

  /**
   * Fetch current authenticated user
   */
  const fetchCurrentUser = useCallback(async () => {
    setLoading(true)
    try {
      const authenticatedUser = await getCurrentUser()
      const userData: User = {
        username: authenticatedUser.username,
        email: authenticatedUser.signInDetails?.loginId || "",
        userId: authenticatedUser.userId,
      }
      setUser(userData)
      return userData
    } catch {
      setUser(null)
      return null
    } finally {
      setLoading(false)
    }
  }, [setLoading, setUser])

  // Fetch authenticated user on initial load
  useEffect(() => {
    fetchCurrentUser()
  }, [fetchCurrentUser])

  /**
   * Register a new user
   */
  const handleSignUp = useCallback(
    async (params: SignUpParameters) => {
      setError(null)
      setLoading(true)
      try {
        await signUp({
          username: params.email,
          password: params.password,
        })

        return true
      } catch (err: unknown) {
        const errorMessage = err instanceof Error ? err.message : "Error signing up"
        setError(errorMessage)
        throw err
      } finally {
        setLoading(false)
      }
    },
    [setError, setLoading],
  )

  /**
   * Sign in an existing user
   */
  const handleSignIn = useCallback(
    async (params: SignInInput) => {
      setError(null)
      setLoading(true)
      try {
        const { isSignedIn } = await signIn({
          username: params.email,
          password: params.password,
        })
        if (isSignedIn) {
          // Fetch user info and update state
          const userData = await fetchCurrentUser()
          if (!userData) return false

          // Get tokens and send to backend
          const { idToken, refreshToken } = storageRef.current.getTokens()

          // Send tokens to backend
          await postSignIn({
            data: {
              id_token: idToken,
              refresh_token: refreshToken,
            },
          })

          return true
        }
        return false
      } catch (err: unknown) {
        const errorMessage = err instanceof Error ? err.message : "Error signing in"
        setError(errorMessage)
        throw err
      } finally {
        setLoading(false)
      }
    },
    [fetchCurrentUser, postSignIn, setError, setLoading],
  )

  /**
   * Sign out the current user
   */
  const handleSignOut = useCallback(async () => {
    setError(null)
    setLoading(true)
    try {
      await signOut({ global: true })
      setUser(null)
      return true
    } catch (err: unknown) {
      const errorMessage = err instanceof Error ? err.message : "Error signing out"
      setError(errorMessage)
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
    fetchCurrentUser,
  }
}
