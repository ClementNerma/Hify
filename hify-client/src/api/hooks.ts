import { use, useRef, useState } from 'react'
import { showFailure, showNotification } from '#/global/notifications.ts'
import { fail, getOrInsertWith, staticTypeAssert } from '#/utils/common.ts'
import { useValueIdentityWatcher, useValueWatcher } from '#/utils/hooks.ts'
import type { Paginated, Pagination, PaginationDir } from './types'

//
// => Queries
//

const queriesCache = new Map<string, Promise<unknown>>()

type CachableQuery<T> = { queryKey: string[]; queryFn: () => Promise<T> }

export function useSuspenseQuery<T>({ queryKey: queryKeyArray, queryFn }: CachableQuery<T>): T {
  const queryKey = queryKeyArray.join('|')

  // oxlint-disable-next-line typescript/no-unsafe-type-assertion
  const retypedCache = staticTypeAssert<Map<string, Promise<unknown>>>(queriesCache) as Map<
    string,
    Promise<T>
  >

  const initialQueryKey = useRef(queryKey)

  return use(getOrInsertWith(retypedCache, initialQueryKey.current, queryFn))
}

export function useSuspenseQueries<Q extends CachableQuery<unknown>[]>(
  ...queries: Q
): ApiQueriesOutput<Q> {
  return useSuspenseQuery({
    queryKey: ['#suspenseQueries', ...queries.map(({ queryKey }) => queryKey.join('|'))],

    queryFn: () => {
      const promise = Promise.all(queries.map((q) => q.queryFn()))

      // oxlint-disable-next-line typescript/no-unsafe-type-assertion
      return promise as Promise<ApiQueriesOutput<Q>>
    },
  })
}

type ApiQueriesOutput<Q> = {
  [P in keyof Q]: Q[P] extends CachableQuery<infer T> ? T : never
}

const suspensePaginatedQueriesCache = new Map<string, Promise<unknown>>()

type PaginatedQueryOutput<T> = {
  data: T[] | null
  fetchNextPage: () => void
  hasMore: boolean
  isLoading: boolean
}

export function usePaginatedQuery<T>({
  queryKey: queryKeyArray,
  queryFn,
  paginationDir,
  pageSize,
  suspense,
}: {
  queryKey: string[]
  queryFn: (pagination: Pagination) => Promise<Paginated<T>>
  paginationDir: PaginationDir
  pageSize: number
  suspense?: boolean
}): PaginatedQueryOutput<T> {
  const queryKey = `${queryKeyArray.join('|')}:${paginationDir}:${pageSize}`

  type InitialState = { results: T[] | null; hasMore: boolean }

  // oxlint-disable-next-line typescript/no-unsafe-type-assertion
  const retypedCache = staticTypeAssert<Map<string, Promise<unknown>>>(
    suspensePaginatedQueriesCache,
  ) as Map<string, Promise<InitialState>>

  const initialQueryKey = useRef(queryKey)

  const initialState = use(
    getOrInsertWith(retypedCache, initialQueryKey.current, async () =>
      suspense === true
        ? queryFn({ offset: 0, limit: pageSize, dir: paginationDir })
        : { results: null, hasMore: true },
    ),
  )

  const [data, setData] = useState<T[] | null>(initialState.results)
  const [hasMore, setHasMore] = useState(initialState.hasMore)
  const [isLoading, setIsLoading] = useState(false)

  const offset = useRef(initialState.results ? initialState.results.length : 0)

  const fetchNextPage = async () => {
    if (isLoading) {
      showFailure(
        'Cannot fetch the next page from paginated query while the current one is already being fetched',
        'warning',
      )

      return
    }

    if (!hasMore) {
      return
    }

    setIsLoading(true)

    const paginated = await queryFn({ offset: offset.current, limit: pageSize, dir: paginationDir })

    setData((prevData) => [...(prevData ?? []), ...paginated.results])
    setHasMore(paginated.hasMore)

    offset.current += paginated.results.length

    setIsLoading(false)
  }

  useValueWatcher(queryKey, () => {
    // // Clear the cache entry to trigger GC
    // retypedCache.set(initialQueryKey.current, Promise.resolve({ results: null, hasMore: true }))

    setData(null)
    setHasMore(true)
    offset.current = 0

    // oxlint-disable-next-line typescript/no-floating-promises
    fetchNextPage()
  })

  return {
    data,
    fetchNextPage: () => {
      // oxlint-disable-next-line typescript/no-floating-promises
      fetchNextPage()
    },
    hasMore,
    isLoading,
  }
}

export function useSuspensePaginatedQuery<T>({
  queryKey,
  queryFn,
  paginationDir,
  pageSize,
}: {
  queryKey: string[]
  queryFn: (pagination: Pagination) => Promise<Paginated<T>>
  pageSize: number
  paginationDir: PaginationDir
}) {
  const [prevData, setPrevData] = useState<T[] | null>(null)

  const { data, isLoading, fetchNextPage } = usePaginatedQuery({
    queryKey,
    queryFn,
    pageSize,
    paginationDir,
    suspense: prevData === null,
  })

  useValueIdentityWatcher(
    data,
    (data) => {
      if (data) {
        setPrevData(data)
      }
    },
    { immediate: true },
  )

  return {
    data: data ?? prevData ?? fail('unexpected'),
    isResetting: !data,
    isLoading,
    fetchNextPage,
  }
}

export function clearQueriesCache(): void {
  queriesCache.clear()
  suspensePaginatedQueriesCache.clear()
}

//
// => Mutations
//

// oxlint-disable-next-line typescript/no-explicit-any
export function useApiMutation<T extends any[]>(
  mutation: (...params: T) => Promise<void>,
): ApiMutationHandler<T> {
  const [status, setStatus] = useState<ApiMutationHandler<T>['status']>('idle')

  const run = (...params: T): void => {
    if (status === 'pending') {
      showFailure('Cannot run the mutation while it is already pending')
      return
    }

    setStatus('pending')

    mutation(...params)
      .then(() => {
        setStatus('success')
      })
      .catch((e: unknown) => {
        setStatus('failed')

        showNotification({
          type: 'error',
          title: 'API Error',
          message: `API mutation failed: ${String(e)}`,
        })
      })
  }

  return {
    status,
    run:
      // oxlint-disable-next-line typescript/no-unsafe-type-assertion
      staticTypeAssert<(...params: T) => void>(run) as ApiMutationHandler<T>['run'],
    reset: () => {
      if (status === 'pending') {
        throw new Error('Cannot reset the mutation while it is pending')
      }

      setStatus('idle')
    },
  }
}

// oxlint-disable-next-line typescript/no-explicit-any
export type ApiMutationHandler<T extends any[]> = {
  status: ApiMutationStatus
  run: T extends void ? () => void : (...params: T) => void
  reset: () => void
}

export type ApiMutationStatus = 'idle' | 'pending' | 'success' | 'failed'
