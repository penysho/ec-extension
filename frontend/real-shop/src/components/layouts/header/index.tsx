"use client"

import { LogIn, Search, ShoppingCart, User } from "lucide-react"
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
    <header className="bg-white shadow-md">
      <div className="container mx-auto flex items-center justify-between py-4 px-6">
        {/* Logo */}
        <div className="logo">
          <Link href="/">
            <Image
              className="dark:invert"
              src="/logo.png"
              alt="logo"
              width={30}
              height={30}
              priority
            />
          </Link>
        </div>

        {/* Navigation */}
        <nav className="hidden md:flex space-x-6">
          <Link
            href="/products?men"
            className="text-gray-600 hover:text-gray-800"
          >
            Men
          </Link>
          <Link
            href="/products?women"
            className="text-gray-600 hover:text-gray-800"
          >
            Women
          </Link>
          <Link
            href="/products?sale"
            className="text-red-600 hover:text-red-800"
          >
            Sale
          </Link>
        </nav>

        {/* Actions */}
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
            <Button asChild variant="default" size="sm">
              <Link href="/login">
                <LogIn className="mr-2 h-4 w-4" />
                <span>ログイン</span>
              </Link>
            </Button>
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
