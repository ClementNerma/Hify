// TODO: properly document everything in this module

import { isKeyOf } from '#/utils/common.ts'

export function parseRoutePath(
  input: string,
  loose?: boolean,
): {
  keys: string[]
  pattern: RegExp
} {
  const keys: string[] = []
  let pattern = ''

  const segments = (input.startsWith('/') ? input.slice(1) : input).split('/')

  while (segments.length > 0) {
    // oxlint-disable-next-line no-non-null-assertion
    const segment = segments.shift()!
    const [firstChar] = segment

    if (firstChar === '*') {
      keys.push(firstChar)
      pattern += segment[1] === '?' ? '(?:/(.*))?' : '/(.*)'
    } else if (firstChar === ':') {
      const o = segment.indexOf('?', 1)
      const ext = segment.indexOf('.', 1)

      keys.push(segment.slice(1, o !== -1 ? o : ext !== -1 ? ext : segment.length))

      pattern += o !== -1 && ext === -1 ? '(?:/([^/]+?))?' : '/([^/]+?)'

      if (ext !== -1) {
        pattern += `${o !== -1 ? '?' : ''}\\${segment.slice(ext)}`
      }
    } else {
      pattern += `/${segment}`
    }
  }

  return {
    keys,
    pattern: new RegExp(`^${pattern}${loose === true ? '(?=$|/)' : '/?$'}`, 'i'),
  }
}

export type RouteUriParams<T extends string> = T extends `${infer Prev}/*/${infer Rest}`
  ? RouteUriParams<Prev> & { wild: string } & RouteUriParams<Rest>
  : T extends `${string}:${infer P}?/${infer Rest}`
    ? { [K in P]?: string } & RouteUriParams<Rest>
    : T extends `${string}:${infer P}/${infer Rest}`
      ? { [K in P]: string } & RouteUriParams<Rest>
      : T extends `${string}:${infer P}?`
        ? { [K in P]?: string }
        : T extends `${string}:${infer P}`
          ? { [K in P]: string }
          : T extends `${string}*`
            ? { '*': string }
            : T extends `${string}*?`
              ? { '*'?: string }
              : Record<string, never>

const ROUTE_PARAMS_REGEX = /(\/|^)([:*][^/]*?)(\?)?(?=[/.]|$)/g

export function injectParamsInRoute<T extends string>(route: T, values: RouteUriParams<T>) {
  // TODO: explain how this works
  return route.replace(
    ROUTE_PARAMS_REGEX,
    (_x: string, _lead: string, rawKey: string, optional: string | undefined) => {
      const key = rawKey === '*' ? rawKey : rawKey.slice(1)

      if (!isKeyOf(values, key) && optional === undefined) {
        throw new Error(`Missing param "${key}" for route "${route}"`)
      }

      const out =
        isKeyOf(values, key) && values[key] !== undefined
          ? // oxlint-disable-next-line typescript/no-unsafe-type-assertion
            encodeURIComponent(values[key] as string)
          : ''

      // TODO: optimize
      return out ? `/${out}` : Boolean(optional) || key === '*' ? '' : `/${out}`
    },
  )
}
