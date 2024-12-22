"use client"

import { LogIn, ShoppingCart, User } from "lucide-react"
import Image from "next/image"
import Link from "next/link"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { useAuth } from "@/hooks/useAuth"

export const Header = () => {
  const { user } = useAuth()

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

        {/* Search */}
        <div className="hidden md:block w-1/3">
          <Input
            type="text"
            placeholder="Search for products"
            className="w-full"
          />
        </div>

        {/* Actions */}
        <div className="flex items-center space-x-4">
          {user ? (
            <Link href="/account" className="text-gray-600 hover:text-gray-800">
              <User className="h-5 w-5" />
              <span className="sr-only">Account</span>
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
            <span className="sr-only">Cart</span>
          </Link>
        </div>
      </div>
    </header>
  )
}
