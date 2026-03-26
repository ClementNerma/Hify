import { injectParamsInRoute, parseRoutePath, type RouteUriParams } from './parser'

export type Route<T extends string> = {
  path: T
  pathPattern: RegExp
  params: string[]
}

export function route<T extends string>(path: T): Route<T> {
  const { keys, pattern } = parseRoutePath(path)

  return {
    path,
    pathPattern: pattern,
    params: keys,
  }
}

export type UntypedRoutes = Record<string, Route<string>>

export function buildRoutes<R extends UntypedRoutes>(routes: R): R {
  return routes
}

export function getRouteUri<T extends string>(route: Route<T>, params: RouteUriParams<T>): string {
  return injectParamsInRoute(route.path, params)
}

export type RouteParams<R extends Route<string>> = RouteUriParams<
  R extends Route<infer T> ? T : never
>

export function navigate<R extends Route<string>>(
  route: R,
  params: RouteUriParams<R extends Route<infer P> ? P : never>,
) {
  history.pushState(
    {},
    '',
    // oxlint-disable-next-line no-explicit-any, no-unsafe-argument, no-unsafe-type-assertion
    injectParamsInRoute(route.path, params as any),
  )

  window.dispatchEvent(new Event('custompushstate'))
}
