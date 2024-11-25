import Image from "next/image"

type ProductImageProps = {
  url: string
}

export default function ProductImage({ url }: ProductImageProps) {
  return (
    <div className="relative">
      <Image
        src={url}
        alt="Product Image"
        height={600}
        width={600}
        className="w-full h-auto rounded-lg shadow-lg"
      />
    </div>
  )
}
