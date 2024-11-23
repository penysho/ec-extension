import Image from "next/image"

export const Header = () => {
  return (
    <header className="header bg-white shadow-md">
      <div className="container mx-auto flex items-center justify-between py-4 px-6">
        {/* Logo */}
        <div className="logo">
          <a href="/">
            <Image
              className="dark:invert"
              src="/next.svg"
              alt="Next.js logo"
              width={180}
              height={38}
              priority
            />
          </a>
        </div>

        {/* Navigation */}
        <nav className="navigation hidden md:flex space-x-6">
          <a href="/men" className="text-gray-600 hover:text-gray-800">
            Men
          </a>
          <a href="/women" className="text-gray-600 hover:text-gray-800">
            Women
          </a>
          <a href="/sale" className="text-red-600 hover:text-red-800">
            Sale
          </a>
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
          <a href="/cart" className="text-gray-600 hover:text-gray-800">
            <i className="fas fa-shopping-cart"></i> Cart
          </a>
          <a href="/account" className="text-gray-600 hover:text-gray-800">
            <i className="fas fa-user"></i> Account
          </a>
        </div>
      </div>
    </header>
  )
}
