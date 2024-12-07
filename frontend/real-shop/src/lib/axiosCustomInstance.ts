// eslint-disable-next-line import/no-named-as-default
import Axios, { AxiosError, AxiosRequestConfig } from "axios"

export const AXIOS_INSTANCE = Axios.create({
  baseURL: process.env.NEXT_PUBLIC_BACKEND_ENDPOINT,
  withCredentials: true,
})

export const customInstance = <T>(
  config: AxiosRequestConfig,
  options?: AxiosRequestConfig,
): Promise<T> => {
  // eslint-disable-next-line import/no-named-as-default-member
  const source = Axios.CancelToken.source()
  const promise = AXIOS_INSTANCE({
    ...config,
    ...options,
    cancelToken: source.token,
  }).then(({ data }) => data)

  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-expect-error
  promise.cancel = () => {
    source.cancel("Query was cancelled")
  }

  return promise
}

export type ErrorType<Error> = AxiosError<Error>

export type BodyType<BodyData> = BodyData
