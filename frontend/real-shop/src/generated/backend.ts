/**
 * Generated by orval v7.3.0 🍺
 * Do not edit manually.
 * ec-extension_backend
 * OpenAPI spec version: 0.0.1
 */
import { useQuery } from "@tanstack/react-query"
import type {
  DataTag,
  DefinedInitialDataOptions,
  DefinedUseQueryResult,
  QueryClient,
  QueryFunction,
  QueryKey,
  UndefinedInitialDataOptions,
  UseQueryOptions,
  UseQueryResult,
} from "@tanstack/react-query"
import { customInstance } from "../lib/axiosCustomInstance"
import type { ErrorType } from "../lib/axiosCustomInstance"
export type GetCustomersParams = {
  /**
   * email
   */
  email?: string
}

export type GetProductsParams = {
  /**
   * limit
   */
  limit?: number
  /**
   * offset
   */
  offset?: number
}

/**
 * Not found
 */
export type NotFoundResponse = DomainError

/**
 * Get a list of customers resoponse
 */
export type GetCustomersResponseResponse = {
  customers: Customer[]
}

/**
 * Get a list of products resoponse
 */
export type GetProductsResponseResponse = {
  products: Product[]
}

/**
 * Get detailed product information resoponse
 */
export type GetProductResponseResponse = {
  product: Product
}

/**
 * Common Error Responses
 */
export interface DomainError {
  message: string
}

/**
 * Service unavailable
 */
export type ServiceUnavailableResponse = DomainError

/**
 * Internal server error
 */
export type InternalServerErrorResponse = DomainError

/**
 * Bad request
 */
export type BadRequestResponse = DomainError

export interface Address {
  /**
   * The primary address line.
   * @nullable
   */
  address1?: string | null
  /**
   * The secondary address line, such as apartment or suite number.
   * @nullable
   */
  address2?: string | null
  /**
   * The city of the address.
   * @nullable
   */
  city?: string | null
  /** Indicates if the address coordinates are validated. */
  coordinates_validated: boolean
  /**
   * The country of the address.
   * @nullable
   */
  country?: string | null
  /**
   * The first name associated with the address.
   * @nullable
   */
  first_name?: string | null
  /**
   * The last name associated with the address.
   * @nullable
   */
  last_name?: string | null
  /**
   * The phone number associated with the address.
   * @nullable
   */
  phone?: string | null
  /**
   * The state or province of the address.
   * @nullable
   */
  province?: string | null
  /**
   * The postal code of the address.
   * @nullable
   */
  zip?: string | null
}

/**
 * The status of the customer.
 */
export type CustomerStatus =
  (typeof CustomerStatus)[keyof typeof CustomerStatus]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const CustomerStatus = {
  Active: "Active",
  Inactive: "Inactive",
} as const

export interface Image {
  /**
   * The alternative text for the image.
   * @nullable
   */
  alt?: string | null
  /** The unique identifier for the image. */
  id: string
  /**
   * The source URL of the image.
   * @nullable
   */
  src?: string | null
}

export interface Customer {
  /** A list of addresses associated with the customer. */
  addresses: Address[]
  /** The timestamp when the customer was created. */
  created_at: string
  /** The default address of the customer. */
  default_address?: Address
  /** The display name of the customer. */
  display_name: string
  /**
   * The email address of the customer.
   * @nullable
   */
  email?: string | null
  /**
   * The first name of the customer.
   * @nullable
   */
  first_name?: string | null
  /** The unique identifier of the customer. */
  id: string
  /** An image associated with the customer. */
  image?: Image
  /**
   * The last name of the customer.
   * @nullable
   */
  last_name?: string | null
  /**
   * A note associated with the customer.
   * @nullable
   */
  note?: string | null
  /**
   * The phone number of the customer.
   * @nullable
   */
  phone?: string | null
  status: CustomerStatus
  /** The timestamp when the customer was last updated. */
  updated_at: string
  /** Indicates if the customer's email is verified. */
  verified_email: boolean
}

export interface MediaContent {
  /** The image content associated with the media. */
  image?: Image
}

/**
 * The status of the media.
 */
export type MediaStatus = (typeof MediaStatus)[keyof typeof MediaStatus]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const MediaStatus = {
  Active: "Active",
  Inactive: "Inactive",
  InPreparation: "InPreparation",
} as const

export interface Media {
  content?: MediaContent
  /** The timestamp when the media was created. */
  created_at: string
  /** The unique identifier for the media. */
  id: string
  /**
   * The name of the media file.
   * @nullable
   */
  name?: string | null
  status: MediaStatus
  /** The timestamp when the media was last updated. */
  updated_at: string
}

/**
 * The policy for inventory management.
 */
export type InventoryPolicy =
  (typeof InventoryPolicy)[keyof typeof InventoryPolicy]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const InventoryPolicy = {
  Deny: "Deny",
  Continue: "Continue",
} as const

export interface Variant {
  /** Whether the variant is available for sale. */
  available_for_sale: boolean
  /**
   * The barcode of the variant.
   * @nullable
   */
  barcode?: string | null
  /** The timestamp when the variant was created. */
  created_at: string
  /** The unique identifier for the variant. */
  id: string
  /** The unique identifier for the inventory item. */
  inventory_item_id: string
  inventory_policy: InventoryPolicy
  /**
   * The quantity available in inventory.
   * @nullable
   */
  inventory_quantity?: number | null
  /** The position of the variant in a list. */
  list_order: number
  /**
   * The name of the variant.
   * @nullable
   */
  name?: string | null
  /** The price of the variant. */
  price: number
  /**
   * The stock keeping unit of the variant.
   * @nullable
   */
  sku?: string | null
  /**
   * The tax code applicable to the variant.
   * @nullable
   */
  tax_code?: string | null
  /** Whether the variant is taxable. */
  taxable: boolean
  /** The timestamp when the variant was last updated. */
  updated_at: string
}

/**
 * The status of the product.
 */
export type ProductStatus = (typeof ProductStatus)[keyof typeof ProductStatus]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ProductStatus = {
  Active: "Active",
  Inactive: "Inactive",
  Draft: "Draft",
} as const

export interface Product {
  /**
   * The ID of the category to which the product belongs.
   * @nullable
   */
  category_id?: string | null
  /** A detailed description of the product. */
  description: string
  /** The unique identifier for the product. */
  id: string
  /** A list of media associated with the product. */
  media: Media[]
  /** The name of the product. */
  name: string
  status: ProductStatus
  /** A list of variants of the product. */
  variants: Variant[]
}

type SecondParameter<T extends (...args: any) => any> = Parameters<T>[1]

/**
 * Get detailed product information
 * @summary Get detailed product information
 */
export const getProduct = (
  id: number,
  options?: SecondParameter<typeof customInstance>,
  signal?: AbortSignal,
) => {
  return customInstance<GetProductResponseResponse>(
    { url: `/ec-extension/products/${id}`, method: "GET", signal },
    options,
  )
}

export const getGetProductQueryKey = (id: number) => {
  return [`/ec-extension/products/${id}`] as const
}

export const getGetProductQueryOptions = <
  TData = Awaited<ReturnType<typeof getProduct>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  id: number,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProduct>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
) => {
  const { query: queryOptions, request: requestOptions } = options ?? {}

  const queryKey = queryOptions?.queryKey ?? getGetProductQueryKey(id)

  const queryFn: QueryFunction<Awaited<ReturnType<typeof getProduct>>> = ({
    signal,
  }) => getProduct(id, requestOptions, signal)

  return {
    queryKey,
    queryFn,
    enabled: !!id,
    ...queryOptions,
  } as UseQueryOptions<
    Awaited<ReturnType<typeof getProduct>>,
    TError,
    TData
  > & { queryKey: DataTag<QueryKey, TData> }
}

export type GetProductQueryResult = NonNullable<
  Awaited<ReturnType<typeof getProduct>>
>
export type GetProductQueryError = ErrorType<
  | BadRequestResponse
  | NotFoundResponse
  | InternalServerErrorResponse
  | ServiceUnavailableResponse
>

export function useGetProduct<
  TData = Awaited<ReturnType<typeof getProduct>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  id: number,
  options: {
    query: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProduct>>, TError, TData>
    > &
      Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof getProduct>>,
          TError,
          TData
        >,
        "initialData"
      >
    request?: SecondParameter<typeof customInstance>
  },
): DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
export function useGetProduct<
  TData = Awaited<ReturnType<typeof getProduct>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  id: number,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProduct>>, TError, TData>
    > &
      Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof getProduct>>,
          TError,
          TData
        >,
        "initialData"
      >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
export function useGetProduct<
  TData = Awaited<ReturnType<typeof getProduct>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  id: number,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProduct>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
/**
 * @summary Get detailed product information
 */

export function useGetProduct<
  TData = Awaited<ReturnType<typeof getProduct>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  id: number,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProduct>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> } {
  const queryOptions = getGetProductQueryOptions(id, options)

  const query = useQuery(queryOptions) as UseQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData>
  }

  query.queryKey = queryOptions.queryKey

  return query
}

/**
 * @summary Get detailed product information
 */
export const prefetchGetProduct = async <
  TData = Awaited<ReturnType<typeof getProduct>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  queryClient: QueryClient,
  id: number,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProduct>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): Promise<QueryClient> => {
  const queryOptions = getGetProductQueryOptions(id, options)

  await queryClient.prefetchQuery(queryOptions)

  return queryClient
}

/**
 * Get a list of products
 * @summary Get a list of products
 */
export const getProducts = (
  params?: GetProductsParams,
  options?: SecondParameter<typeof customInstance>,
  signal?: AbortSignal,
) => {
  return customInstance<GetProductsResponseResponse>(
    { url: `/ec-extension/products`, method: "GET", params, signal },
    options,
  )
}

export const getGetProductsQueryKey = (params?: GetProductsParams) => {
  return [`/ec-extension/products`, ...(params ? [params] : [])] as const
}

export const getGetProductsQueryOptions = <
  TData = Awaited<ReturnType<typeof getProducts>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params?: GetProductsParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProducts>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
) => {
  const { query: queryOptions, request: requestOptions } = options ?? {}

  const queryKey = queryOptions?.queryKey ?? getGetProductsQueryKey(params)

  const queryFn: QueryFunction<Awaited<ReturnType<typeof getProducts>>> = ({
    signal,
  }) => getProducts(params, requestOptions, signal)

  return { queryKey, queryFn, ...queryOptions } as UseQueryOptions<
    Awaited<ReturnType<typeof getProducts>>,
    TError,
    TData
  > & { queryKey: DataTag<QueryKey, TData> }
}

export type GetProductsQueryResult = NonNullable<
  Awaited<ReturnType<typeof getProducts>>
>
export type GetProductsQueryError = ErrorType<
  | BadRequestResponse
  | NotFoundResponse
  | InternalServerErrorResponse
  | ServiceUnavailableResponse
>

export function useGetProducts<
  TData = Awaited<ReturnType<typeof getProducts>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params: undefined | GetProductsParams,
  options: {
    query: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProducts>>, TError, TData>
    > &
      Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof getProducts>>,
          TError,
          TData
        >,
        "initialData"
      >
    request?: SecondParameter<typeof customInstance>
  },
): DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
export function useGetProducts<
  TData = Awaited<ReturnType<typeof getProducts>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params?: GetProductsParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProducts>>, TError, TData>
    > &
      Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof getProducts>>,
          TError,
          TData
        >,
        "initialData"
      >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
export function useGetProducts<
  TData = Awaited<ReturnType<typeof getProducts>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params?: GetProductsParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProducts>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
/**
 * @summary Get a list of products
 */

export function useGetProducts<
  TData = Awaited<ReturnType<typeof getProducts>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params?: GetProductsParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProducts>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> } {
  const queryOptions = getGetProductsQueryOptions(params, options)

  const query = useQuery(queryOptions) as UseQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData>
  }

  query.queryKey = queryOptions.queryKey

  return query
}

/**
 * @summary Get a list of products
 */
export const prefetchGetProducts = async <
  TData = Awaited<ReturnType<typeof getProducts>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  queryClient: QueryClient,
  params?: GetProductsParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getProducts>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): Promise<QueryClient> => {
  const queryOptions = getGetProductsQueryOptions(params, options)

  await queryClient.prefetchQuery(queryOptions)

  return queryClient
}

/**
 * Get a list of customers
 * @summary Get a list of customers
 */
export const getCustomers = (
  params?: GetCustomersParams,
  options?: SecondParameter<typeof customInstance>,
  signal?: AbortSignal,
) => {
  return customInstance<GetCustomersResponseResponse>(
    { url: `/ec-extension/customers`, method: "GET", params, signal },
    options,
  )
}

export const getGetCustomersQueryKey = (params?: GetCustomersParams) => {
  return [`/ec-extension/customers`, ...(params ? [params] : [])] as const
}

export const getGetCustomersQueryOptions = <
  TData = Awaited<ReturnType<typeof getCustomers>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params?: GetCustomersParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getCustomers>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
) => {
  const { query: queryOptions, request: requestOptions } = options ?? {}

  const queryKey = queryOptions?.queryKey ?? getGetCustomersQueryKey(params)

  const queryFn: QueryFunction<Awaited<ReturnType<typeof getCustomers>>> = ({
    signal,
  }) => getCustomers(params, requestOptions, signal)

  return { queryKey, queryFn, ...queryOptions } as UseQueryOptions<
    Awaited<ReturnType<typeof getCustomers>>,
    TError,
    TData
  > & { queryKey: DataTag<QueryKey, TData> }
}

export type GetCustomersQueryResult = NonNullable<
  Awaited<ReturnType<typeof getCustomers>>
>
export type GetCustomersQueryError = ErrorType<
  | BadRequestResponse
  | NotFoundResponse
  | InternalServerErrorResponse
  | ServiceUnavailableResponse
>

export function useGetCustomers<
  TData = Awaited<ReturnType<typeof getCustomers>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params: undefined | GetCustomersParams,
  options: {
    query: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getCustomers>>, TError, TData>
    > &
      Pick<
        DefinedInitialDataOptions<
          Awaited<ReturnType<typeof getCustomers>>,
          TError,
          TData
        >,
        "initialData"
      >
    request?: SecondParameter<typeof customInstance>
  },
): DefinedUseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
export function useGetCustomers<
  TData = Awaited<ReturnType<typeof getCustomers>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params?: GetCustomersParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getCustomers>>, TError, TData>
    > &
      Pick<
        UndefinedInitialDataOptions<
          Awaited<ReturnType<typeof getCustomers>>,
          TError,
          TData
        >,
        "initialData"
      >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
export function useGetCustomers<
  TData = Awaited<ReturnType<typeof getCustomers>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params?: GetCustomersParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getCustomers>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> }
/**
 * @summary Get a list of customers
 */

export function useGetCustomers<
  TData = Awaited<ReturnType<typeof getCustomers>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  params?: GetCustomersParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getCustomers>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): UseQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData> } {
  const queryOptions = getGetCustomersQueryOptions(params, options)

  const query = useQuery(queryOptions) as UseQueryResult<TData, TError> & {
    queryKey: DataTag<QueryKey, TData>
  }

  query.queryKey = queryOptions.queryKey

  return query
}

/**
 * @summary Get a list of customers
 */
export const prefetchGetCustomers = async <
  TData = Awaited<ReturnType<typeof getCustomers>>,
  TError = ErrorType<
    | BadRequestResponse
    | NotFoundResponse
    | InternalServerErrorResponse
    | ServiceUnavailableResponse
  >,
>(
  queryClient: QueryClient,
  params?: GetCustomersParams,
  options?: {
    query?: Partial<
      UseQueryOptions<Awaited<ReturnType<typeof getCustomers>>, TError, TData>
    >
    request?: SecondParameter<typeof customInstance>
  },
): Promise<QueryClient> => {
  const queryOptions = getGetCustomersQueryOptions(params, options)

  await queryClient.prefetchQuery(queryOptions)

  return queryClient
}
