"use client"
import {
  isServer,
  QueryClient,
  QueryClientProvider,
} from "@tanstack/react-query"
import axios from "axios"

function makeQueryClient() {
  return new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 60 * 1000,
      },
    },
  })
}

let browserQueryClient: QueryClient | undefined = undefined

function getQueryClient() {
  if (isServer) {
    return makeQueryClient()
  } else {
    if (!browserQueryClient) browserQueryClient = makeQueryClient()
    return browserQueryClient
  }
}

export default function ReactQueryProvider({
  children,
}: {
  children: React.ReactNode
}) {
  const queryClient = getQueryClient()
  axios.defaults.baseURL = process.env.NEXT_PUBLIC_BACKEND_ENDPOINT

  return (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
}
