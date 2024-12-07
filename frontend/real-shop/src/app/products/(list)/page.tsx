import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query"

import { getProducts } from "@/generated/backend"

import ProductListPresenter from "./presenter"

export default async function Page() {
  const queryClient = new QueryClient()

  await queryClient.prefetchQuery({
    queryKey: ["products"],
    queryFn: async () => {
      const response = await getProducts()
      return response.data
    },
  })

  return (
    <HydrationBoundary state={dehydrate(queryClient)}>
      <ProductListPresenter />
    </HydrationBoundary>
  )
}
