import Link from "next/link"

export const Footer = () => {
  return (
    <footer className="footer bg-gray-800 py-8 text-white">
      <div className="container mx-auto grid grid-cols-1 gap-6 md:grid-cols-3">
        {/* Company Info */}
        <div>
          <h4 className="mb-4 text-lg font-bold">About Us</h4>
          <p>We are a leading online fashion retailer, bringing the latest trends to your doorstep.</p>
        </div>

        {/* Links */}
        <div>
          <h4 className="mb-4 text-lg font-bold">Customer Service</h4>
          <ul className="space-y-2">
            <li>
              <Link href="/shipping" className="hover:underline">
                Shipping Information
              </Link>
            </li>
            <li>
              <Link href="/returns" className="hover:underline">
                Returns & Refunds
              </Link>
            </li>
            <li>
              <Link href="/privacy" className="hover:underline">
                Privacy Policy
              </Link>
            </li>
          </ul>
        </div>

        {/* Newsletter */}
        <div>
          <h4 className="mb-4 text-lg font-bold">Stay Connected</h4>
          <form>
            <input
              type="email"
              placeholder="Enter your email"
              className="mb-2 w-full rounded px-4 py-2 text-gray-800"
            />
            <button type="submit" className="w-full rounded bg-red-600 py-2 text-white hover:bg-red-700">
              Subscribe
            </button>
          </form>
          <div className="mt-4 space-x-4">
            <Link href="https://facebook.com" target="_blank" rel="noopener noreferrer">
              <i className="fab fa-facebook"></i>
            </Link>
            <Link href="https://twitter.com" target="_blank" rel="noopener noreferrer">
              <i className="fab fa-twitter"></i>
            </Link>
            <Link href="https://instagram.com" target="_blank" rel="noopener noreferrer">
              <i className="fab fa-instagram"></i>
            </Link>
          </div>
        </div>
      </div>
    </footer>
  )
}
