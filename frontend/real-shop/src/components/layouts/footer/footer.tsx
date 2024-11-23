export const Footer = () => {
  return (
    <footer className="footer bg-gray-800 text-white py-8">
      <div className="container mx-auto grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* Company Info */}
        <div>
          <h4 className="text-lg font-bold mb-4">About Us</h4>
          <p>
            We are a leading online fashion retailer, bringing the latest trends
            to your doorstep.
          </p>
        </div>

        {/* Links */}
        <div>
          <h4 className="text-lg font-bold mb-4">Customer Service</h4>
          <ul className="space-y-2">
            <li>
              <a href="/shipping" className="hover:underline">
                Shipping Information
              </a>
            </li>
            <li>
              <a href="/returns" className="hover:underline">
                Returns & Refunds
              </a>
            </li>
            <li>
              <a href="/privacy" className="hover:underline">
                Privacy Policy
              </a>
            </li>
          </ul>
        </div>

        {/* Newsletter */}
        <div>
          <h4 className="text-lg font-bold mb-4">Stay Connected</h4>
          <form>
            <input
              type="email"
              placeholder="Enter your email"
              className="w-full mb-2 px-4 py-2 text-gray-800 rounded"
            />
            <button
              type="submit"
              className="w-full bg-red-600 hover:bg-red-700 text-white py-2 rounded"
            >
              Subscribe
            </button>
          </form>
          <div className="mt-4 space-x-4">
            <a
              href="https://facebook.com"
              target="_blank"
              rel="noopener noreferrer"
            >
              <i className="fab fa-facebook"></i>
            </a>
            <a
              href="https://twitter.com"
              target="_blank"
              rel="noopener noreferrer"
            >
              <i className="fab fa-twitter"></i>
            </a>
            <a
              href="https://instagram.com"
              target="_blank"
              rel="noopener noreferrer"
            >
              <i className="fab fa-instagram"></i>
            </a>
          </div>
        </div>
      </div>
    </footer>
  )
}
