import { ReactQueryDevtools } from "@tanstack/react-query-devtools"
import axios from "axios"
import { Provider as StateProvider } from "jotai"
import { Metadata } from "next"
import { Inter } from "next/font/google"

import { Footer } from "@/components/layouts/footer"
import { Header } from "@/components/layouts/header"
import ReactQueryProvider from "@/lib/ReactQueryProvider"

import "@aws-amplify/ui-react/styles.css"
import "./globals.css"

const inter = Inter({ subsets: ["latin"] })

export const metadata: Metadata = {
  title: "Real Shop Customer Service",
  description: "We deliver the right product for you from EC.",
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  axios.defaults.baseURL = process.env.NEXT_PUBLIC_BACKEND_ENDPOINT

  return (
    <html lang="ja">
      <body className={inter.className}>
        <ReactQueryProvider>
          <StateProvider>
            <Header />
            <ReactQueryDevtools initialIsOpen={false} />
            <main>{children}</main>
            <Footer />
          </StateProvider>
        </ReactQueryProvider>
      </body>
    </html>
  )
}
