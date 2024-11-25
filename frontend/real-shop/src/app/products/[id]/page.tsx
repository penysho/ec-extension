import ProductImage from "@/components/elements/productImage"

export default function ProductDetail() {
  const product = {
    id: "123",
    name: "Modern Jacket",
    description: "A stylish and comfortable jacket for everyday wear.",
    price: 12000,
    image: "https://placehold.jp/600x600.png",
    relatedProducts: [
      { id: "124", name: "Casual Shirt", image: "/images/shirt.jpg" },
      { id: "125", name: "Leather Shoes", image: "/images/shoes.jpg" },
    ],
  }

  return (
    <div className="container mx-auto px-4 py-6">
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* 商品画像 */}
        <div className="relative">
          <ProductImage url={product.image} />
        </div>

        {/* 商品情報 */}
        <div className="space-y-4">
          <h1 className="text-3xl font-bold text-gray-800">{product.name}</h1>
          <p className="text-gray-600">{product.description}</p>
          <p className="text-2xl font-semibold text-gray-800">
            ¥{product.price.toLocaleString()}
          </p>
          <button className="px-6 py-2 bg-blue-600 text-white rounded-lg shadow hover:bg-blue-700 transition">
            カートに追加
          </button>
        </div>
      </div>
    </div>
  )
}
