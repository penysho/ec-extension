import { ReactQueryDevtools } from "@tanstack/react-query-devtools"
import { Provider as StateProvider } from "jotai"
import { Metadata } from "next"
import { Inter } from "next/font/google"

import { Footer } from "@/components/layout/footer"
import { Header } from "@/components/layout/header"
import CloudWatchRumProvider from "@/lib/CloudWatchRumProvider"
import MswProvider from "@/lib/MswProvider"
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
  return (
    <html lang="ja">
      <body className={`${inter.className} flex min-h-screen flex-col`}>
        <CloudWatchRumProvider />
        <MswProvider>
          <ReactQueryProvider>
            <StateProvider>
              <Header />
              <main className="flex-grow">{children}</main>
              <Footer />
              <ReactQueryDevtools initialIsOpen={false} />
            </StateProvider>
          </ReactQueryProvider>
        </MswProvider>
      </body>
    </html>
  )
}
