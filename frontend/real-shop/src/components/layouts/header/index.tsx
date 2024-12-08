import Image from "next/image"
import Link from "next/link"

export const Header = () => {
  return (
    <header className="header bg-white shadow-md">
      <div className="container mx-auto flex items-center justify-between py-4 px-6">
        {/* Logo */}
        <div className="logo">
          <Link href="/">
            <Image
              className="dark:invert"
              src="/next.svg"
              alt="Next.js logo"
              width={180}
              height={38}
              priority
            />
          </Link>
        </div>

        {/* Navigation */}
        <nav className="navigation hidden md:flex space-x-6">
          <Link href="/men" className="text-gray-600 hover:text-gray-800">
            Men
          </Link>
          <Link href="/women" className="text-gray-600 hover:text-gray-800">
            Women
          </Link>
          <Link href="/sale" className="text-red-600 hover:text-red-800">
            Sale
          </Link>
        </nav>

        {/* Search */}
        <div className="search hidden md:block">
          <input
            type="text"
            placeholder="Search for products"
            className="border border-gray-300 rounded px-4 py-2"
          />
        </div>

        {/* Actions */}
        <div className="actions flex items-center space-x-4">
          <Link href="/cart" className="text-gray-600 hover:text-gray-800">
            <i className="fas fa-shopping-cart"></i> Cart
          </Link>
          <Link href="/account" className="text-gray-600 hover:text-gray-800">
            <i className="fas fa-user"></i> Account
          </Link>
        </div>
      </div>
    </header>
  )
}
