"use client"

import { Search, ShoppingCart, User } from "lucide-react"
import Image from "next/image"
import Link from "next/link"
import { useState } from "react"

import { Button } from "@/components/ui/button"
import { useAuth } from "@/hooks/useAuth"

import { SearchModal } from "./SearchModal"

export const Header = () => {
  const { user } = useAuth()
  const [isSearchOpen, setIsSearchOpen] = useState(false)

  return (
    <header className="bg-white relative">
      <div className="absolute top-5 left-1/2 -translate-x-1/2">
        <Link href="/">
          <Image
            className="dark:invert"
            src="/logo.svg"
            alt="logo"
            width={140}
            height={140}
            priority
          />
        </Link>
      </div>
      <div className="container mx-auto flex items-center justify-end py-4 px-6 h-20">
        <div className="flex items-center space-x-4">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setIsSearchOpen(true)}
            aria-label="商品を検索"
          >
            <Search className="h-5 w-5" />
          </Button>
          {user ? (
            <Link href="/account" className="text-gray-600 hover:text-gray-800">
              <User className="h-5 w-5" />
              <span className="sr-only">アカウント</span>
            </Link>
          ) : (
            <Link href="/login" className="text-gray-600 hover:text-gray-800">
              <User className="h-5 w-5" />
              <span className="sr-only">ログイン</span>
            </Link>
          )}
          <Link href="/cart" className="text-gray-600 hover:text-gray-800">
            <ShoppingCart className="h-5 w-5" />
            <span className="sr-only">カート</span>
          </Link>
        </div>
      </div>
      <SearchModal
        isOpen={isSearchOpen}
        onClose={() => setIsSearchOpen(false)}
      />
    </header>
  )
}
