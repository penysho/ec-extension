import { Checkbox } from "@/components/ui/checkbox"
import { Label } from "@/components/ui/label"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"

interface SidebarProps {
  categories: string[]
  selectedCategories: string[]
  onCategoryChange: (category: string) => void
  sortOption: string
  onSortChange: (option: string) => void
}

export function Sidebar({
  categories,
  selectedCategories,
  onCategoryChange,
  sortOption,
  onSortChange,
}: SidebarProps) {
  return (
    <div className="w-64 p-4 border-r">
      <h2 className="font-bold text-lg mb-4">フィルター</h2>
      <div className="mb-6">
        <h3 className="font-semibold mb-2">カテゴリー</h3>
        {categories.map((category) => (
          <div key={category} className="flex items-center space-x-2 mb-2">
            <Checkbox
              id={category}
              checked={selectedCategories.includes(category)}
              onCheckedChange={() => onCategoryChange(category)}
            />
            <Label htmlFor={category}>{category}</Label>
          </div>
        ))}
      </div>
      <h2 className="font-bold text-lg mb-4">並び替え</h2>
      <RadioGroup value={sortOption} onValueChange={onSortChange}>
        <div className="flex items-center space-x-2 mb-2">
          <RadioGroupItem value="price_asc" id="price_asc" />
          <Label htmlFor="price_asc">価格: 安い順</Label>
        </div>
        <div className="flex items-center space-x-2 mb-2">
          <RadioGroupItem value="price_desc" id="price_desc" />
          <Label htmlFor="price_desc">価格: 高い順</Label>
        </div>
        <div className="flex items-center space-x-2 mb-2">
          <RadioGroupItem value="newest" id="newest" />
          <Label htmlFor="newest">新着順</Label>
        </div>
      </RadioGroup>
    </div>
  )
}
