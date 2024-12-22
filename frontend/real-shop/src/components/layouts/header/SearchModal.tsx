"use client"

import { Search } from "lucide-react"
import { useState } from "react"

import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { VisuallyHidden } from "@/components/ui/visually-hidden"

interface SearchModalProps {
  isOpen: boolean
  onClose: () => void
}

export function SearchModal({ isOpen, onClose }: SearchModalProps) {
  const [searchQuery, setSearchQuery] = useState("")

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault()
    // ここで検索ロジックを実装
    console.log("Searching for:", searchQuery)
    onClose()
  }

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogTitle>
          <VisuallyHidden>商品検索</VisuallyHidden>
        </DialogTitle>
        <form onSubmit={handleSearch} className="flex items-center space-x-2">
          <Input
            type="text"
            placeholder="商品を検索..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="flex-grow"
          />
          <Button type="submit">
            <Search className="h-4 w-4" />
            <span className="sr-only">検索</span>
          </Button>
        </form>
      </DialogContent>
    </Dialog>
  )
}
