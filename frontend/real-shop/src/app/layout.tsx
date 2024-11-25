import { Metadata } from "next"
import { Inter } from "next/font/google"

import { Footer } from "@/components/layouts/footer/footer"
import { Header } from "@/components/layouts/header/header"
import QueryProvider from "@/libs/queryProvider"
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
  return (
    <html lang="ja">
      <body className={inter.className}>
        <QueryProvider>
          <Header />
          <main>{children}</main>
          <Footer />
        </QueryProvider>
      </body>
    </html>
  )
}
