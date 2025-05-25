import { ExternalLink, Mail } from "lucide-react"
import Link from "next/link"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

export const Footer = () => {
  return (
    <footer className="border-t bg-white">
      <div className="container mx-auto px-6 py-12">
        <div className="grid grid-cols-1 gap-8 md:grid-cols-4">
          {/* Company Info */}
          <div className="md:col-span-2">
            <h4 className="mb-4 text-lg font-semibold text-gray-900">Real Shop</h4>
            <p className="mb-4 text-sm leading-relaxed text-gray-600">
              最新のファッショントレンドをお届けする、
              <br />
              オンラインファッションストア。
              <br />
              高品質な商品を、お手頃価格でご提供いたします。
            </p>
          </div>

          {/* Customer Service Links */}
          <div>
            <h4 className="mb-4 text-sm font-semibold uppercase tracking-wide text-gray-900">カスタマーサービス</h4>
            <ul className="space-y-3">
              <li>
                <Link href="/shipping" className="text-sm text-gray-600 transition-colors hover:text-gray-900">
                  配送について
                </Link>
              </li>
              <li>
                <Link href="/returns" className="text-sm text-gray-600 transition-colors hover:text-gray-900">
                  返品・交換
                </Link>
              </li>
              <li>
                <Link href="/privacy" className="text-sm text-gray-600 transition-colors hover:text-gray-900">
                  プライバシーポリシー
                </Link>
              </li>
              <li>
                <Link href="/terms" className="text-sm text-gray-600 transition-colors hover:text-gray-900">
                  利用規約
                </Link>
              </li>
            </ul>
          </div>

          {/* Newsletter */}
          <div>
            <h4 className="mb-4 text-sm font-semibold uppercase tracking-wide text-gray-900">ニュースレター</h4>
            <p className="mb-4 text-sm text-gray-600">最新情報やお得なキャンペーンをお届けします</p>
            <form className="space-y-3">
              <Input type="email" placeholder="メールアドレス" className="text-sm" />
              <Button type="submit" className="w-full bg-gray-900 text-white transition-colors hover:bg-gray-800">
                <Mail className="mr-2 h-4 w-4" />
                登録する
              </Button>
            </form>
          </div>
        </div>

        {/* Bottom section */}
        <div className="mt-12 border-t pt-8">
          <div className="flex flex-col items-center justify-between space-y-4 md:flex-row md:space-y-0">
            {/* Copyright */}
            <p className="text-sm text-gray-500">© 2024 Real Shop. All rights reserved.</p>

            {/* Social Links */}
            <div className="flex space-x-6">
              <Link
                href="https://facebook.com"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center text-sm text-gray-400 transition-colors hover:text-gray-600"
                aria-label="Facebook"
              >
                <ExternalLink className="mr-1 h-4 w-4" />
                Facebook
              </Link>
              <Link
                href="https://twitter.com"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center text-sm text-gray-400 transition-colors hover:text-gray-600"
                aria-label="Twitter"
              >
                <ExternalLink className="mr-1 h-4 w-4" />
                Twitter
              </Link>
              <Link
                href="https://instagram.com"
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center text-sm text-gray-400 transition-colors hover:text-gray-600"
                aria-label="Instagram"
              >
                <ExternalLink className="mr-1 h-4 w-4" />
                Instagram
              </Link>
            </div>
          </div>
        </div>
      </div>
    </footer>
  )
}
