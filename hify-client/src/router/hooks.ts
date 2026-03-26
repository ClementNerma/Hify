import { useState } from 'react'
import { useOnMounted, useOnUnmounted } from '#/utils/hooks.ts'

export function getLocation() {
  return window.location.pathname
}

// TODO: when supported, use the Navigation API inside this hook
export function useLocation(): string {
  const [location, setLocation] = useState(getLocation())

  const watcher = () => {
    setLocation(getLocation())
  }

  useOnMounted(() => {
    window.addEventListener('custompushstate', watcher)
    window.addEventListener('popstate', watcher)
  })

  useOnUnmounted(() => {
    window.removeEventListener('custompushstate', watcher)
    window.removeEventListener('popstate', watcher)
  })

  return location
}

// // TODO: remove
// export function targetedNavigate<R extends UntypedRoutes>(routes: R) {
//   return <K extends keyof R>(
//     routeName: K,
//     params: R[K] extends Route<infer P> ? RouteUriParams<P> : never,
//   ) => {
//     const route = routes[routeName] as Route<string>

//     navigate(
//       route,
//       // oxlint-disable-next-line no-explicit-any, no-unsafe-argument, no-unsafe-type-assertion
//       params as any,
//     )
//   }
// }
