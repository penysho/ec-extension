import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query"

import { prefetchGetProducts } from "@/generated/backend"

import ProductListPresenter from "./presenter"

export default async function Page() {
  const queryClient = new QueryClient()

  await prefetchGetProducts(queryClient)

  return (
    <HydrationBoundary state={dehydrate(queryClient)}>
      <ProductListPresenter />
    </HydrationBoundary>
  )
}
