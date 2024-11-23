import type { Metadata } from "next"

import { Footer } from "@/components/layouts/footer/footer"
import { Header } from "@/components/layouts/header/header"
import "./globals.css"

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
      <Header />
      <main>{children}</main>
      <Footer />
    </html>
  )
}
