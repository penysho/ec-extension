"use client"

import { ChevronLeft, ChevronRight } from "lucide-react"
import Image from "next/image"
import { useState } from "react"

import { Button } from "@/components/ui/button"

interface ProductGalleryProps {
  images: Array<{
    src: string
    alt?: string
  }>
}

export function ProductGallery({ images }: ProductGalleryProps) {
  const [currentImage, setCurrentImage] = useState(0)

  const nextImage = () => {
    setCurrentImage((prev) => (prev + 1) % images.length)
  }

  const previousImage = () => {
    setCurrentImage((prev) => (prev - 1 + images.length) % images.length)
  }

  return (
    <div className="space-y-4">
      <div className="relative aspect-square">
        <Image
          src={images[currentImage].src}
          alt={images[currentImage].alt || "商品画像"}
          fill
          className="rounded-lg object-cover"
        />
        <div className="absolute inset-0 flex items-center justify-between p-4">
          <Button variant="outline" size="icon" onClick={previousImage} className="bg-white/80 hover:bg-white">
            <ChevronLeft className="h-4 w-4" />
          </Button>
          <Button variant="outline" size="icon" onClick={nextImage} className="bg-white/80 hover:bg-white">
            <ChevronRight className="h-4 w-4" />
          </Button>
        </div>
      </div>
      <div className="grid grid-cols-5 gap-2">
        {images.map((image, index) => (
          <button
            key={index}
            onClick={() => setCurrentImage(index)}
            className={`relative aspect-square overflow-hidden rounded-md border-2 ${
              currentImage === index ? "border-blue-500" : "border-transparent"
            }`}
          >
            <Image src={image.src} alt={image.alt || `商品画像 ${index + 1}`} fill className="object-cover" />
          </button>
        ))}
      </div>
    </div>
  )
}
