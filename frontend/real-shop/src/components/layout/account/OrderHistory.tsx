import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"

interface Order {
  id: string
  date: string
  total: number
  status: string
}

interface OrderHistoryProps {
  userId: string
}

// Mock data
const mockOrders: Order[] = [
  { id: "1", date: "2023-06-01", total: 15000, status: "配送済み" },
  { id: "2", date: "2023-05-15", total: 8000, status: "処理中" },
  { id: "3", date: "2023-04-30", total: 12000, status: "配送済み" },
]

export function OrderHistory({ userId }: OrderHistoryProps) {
  // TODO: implements fetch order history
  console.log(userId)
  return (
    <Card>
      <CardHeader>
        <CardTitle>注文履歴</CardTitle>
        <CardDescription>過去の注文内容を確認できます</CardDescription>
      </CardHeader>
      <CardContent>
        {mockOrders.map((order) => (
          <div key={order.id} className="mb-4 rounded border p-4">
            <div className="mb-2 flex items-center justify-between">
              <span className="font-semibold">注文番号: {order.id}</span>
              <span className="text-sm text-gray-500">{order.date}</span>
            </div>
            <div className="flex items-center justify-between">
              <span>合計: ¥{order.total.toLocaleString()}</span>
              <span className="text-sm font-medium text-blue-600">{order.status}</span>
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  )
}
