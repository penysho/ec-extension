"use client"

import { useAuthContext } from "@/context/AuthContext"

export default function Page() {
  const { user, handleSignIn, handleSignOut, loading, error } = useAuthContext()

  const login = async () => {
    try {
      await handleSignIn({ username: "testuser", password: "password123" })
      alert("Logged in successfully")
    } catch (e) {
      console.error(e)
    }
  }

  return (
    <div>
      <h1>Welcome {user?.username || "Guest"}</h1>
      {loading && <p>Loading...</p>}
      {error && <p style={{ color: "red" }}>{error}</p>}
      {user ? (
        <button onClick={handleSignOut}>Sign Out</button>
      ) : (
        <button onClick={login}>Sign In</button>
      )}
    </div>
  )
}
