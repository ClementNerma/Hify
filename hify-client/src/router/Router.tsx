import type React from 'react'
import { Suspense, useMemo, useState, useTransition, type PropsWithChildren } from 'react'
import { useOnBeforeMounted, useOnMounted, useValueWatcher } from '#/utils/hooks.ts'
import { useLocation } from './hooks'
import type { RouteUriParams } from './parser'
import type { Route, UntypedRoutes } from './routes'

export type RouteRenderers<R extends UntypedRoutes> = {
  [N in keyof R]: R[N] extends Route<infer T> ? React.FC<RouteUriParams<T>> : never
}

export type RouterProps<R extends UntypedRoutes> = {
  routes: R
  renderers: RouteRenderers<R>
  fallback: React.FC
  onLoading?: () => void
  onPageReady?: (routeName: keyof R | null) => void
}

// oxlint-disable-next-line max-lines-per-function
export function Router<R extends UntypedRoutes>({
  routes,
  renderers,
  fallback: Fallback,
  onLoading,
  onPageReady: onLoaded,
}: RouterProps<R>) {
  const location = useLocation()
  const [path, setPath] = useState(location)
  const [, startTransition] = useTransition()

  useOnBeforeMounted(() => onLoading?.())

  useValueWatcher(
    location,
    (newPath) => {
      onLoading?.()

      startTransition(() => {
        setPath(newPath)
      })
    },
    { immediate: true },
  )

  const matchedRoute = useMemo(() => {
    for (const [routeName, route] of Object.entries(routes)) {
      const match = path.match(route.pathPattern)

      if (match) {
        return {
          routeName: routeName as keyof R,
          params: Object.fromEntries(
            route.params.map((param, i) => [
              param,
              match[i + 1] !== undefined ? decodeURIComponent(match[i + 1]) : undefined,
            ]),
          ),
        }
      }
    }

    return null
  }, [path, routes])

  // Smoothly scroll back to the top on view change
  const onPageReady = () => {
    console.info(`-> Navigated to: ${path}`)
    window.scrollTo({ top: 0, left: 0 })
    onLoaded?.(matchedRoute ? matchedRoute.routeName : null)
  }

  if (!matchedRoute) {
    return <Fallback />
  }

  const { routeName, params } = matchedRoute

  const Renderer = renderers[routeName]

  return (
    <Suspense>
      <LoadingWrapper key={path} onMounted={onPageReady}>
        {/* oxlint-disable-next-line typescript/no-explicit-any, typescript/no-unsafe-type-assertion */}
        <Renderer {...(params as any)} />
      </LoadingWrapper>
    </Suspense>
  )
}

function LoadingWrapper({ children, onMounted }: PropsWithChildren<{ onMounted: () => void }>) {
  useOnMounted(onMounted)

  return children
}
