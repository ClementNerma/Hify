export function tryFallible<T>(func: () => T): T | Error {
  try {
    return func()
  } catch (e) {
    return e instanceof Error
      ? e
      : typeof e === 'string'
        ? new Error(e)
        : new Error('Unknown error occurred')
  }
}

// TODO: replace with Promise.try()
export async function tryFallibleAsync<T>(func: () => Promise<T>): Promise<T | Error> {
  try {
    return await func()
  } catch (e) {
    return e instanceof Error
      ? e
      : typeof e === 'string'
        ? new Error(e)
        : new Error('Unknown error occurred')
  }
}

export function staticTypeAssert<T>(value: T): T {
  return value
}

export function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = seconds % 60

  const minutesStr = minutes.toString().padStart(2, '0')
  const secsStr = secs.toString().padStart(2, '0')

  if (hours > 0) {
    return `${hours}:${minutesStr}:${secsStr}`
  }

  return `${minutesStr}:${secsStr}`
}

export function randomInt(): number {
  return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
}

export function randomId(): string {
  return randomInt().toString(16)
}

export function arrayWithoutIndex<T>(values: T[], index: number): T[] {
  if (!Object.hasOwn(values, index)) {
    throw new Error(`Out-of-bounds index ${index} in array of legth ${values.length}`)
  }

  return [...values.slice(0, index), ...values.slice(index + 1)]
}

export function unwrapNotNull<T>(value: T | null): T {
  if (value === null) {
    throw new Error('Unexpected null value')
  }

  return value
}

export function unwrapNotUndefined<T>(value: T | undefined): T {
  if (value === undefined) {
    throw new Error('Unexpected undefined value')
  }

  return value
}

export function isKeyOf<T extends object>(obj: T, key: PropertyKey): key is keyof typeof obj {
  return Object.hasOwn(obj, key)
}

export function noParallel<P extends unknown[]>(
  call: (...params: P) => Promise<void>,
): (...params: P) => Promise<void> {
  let inProgress = false

  return async (...params: P) => {
    if (!inProgress) {
      inProgress = true

      await call(...params).finally(() => {
        inProgress = false
      })

      inProgress = false
    }
  }
}

export function asyncSingleton<T>(call: () => Promise<T>, fallbackOnErr: T): () => Promise<T> {
  let initPromise: Promise<T> | null = null

  return () => {
    initPromise ??= call().catch(() => fallbackOnErr)
    return initPromise
  }
}

export function once(call: () => void): () => void {
  let hasBeenCalled = false

  return () => {
    if (!hasBeenCalled) {
      hasBeenCalled = true
      call()
    }
  }
}

// export function unwrapInstanceOf<T>(
//   value: unknown,
//   // oxlint-disable-next-line typescript/no-explicit-any
//   type: new (...args: any[]) => T,
// ): T {
//   if (!(value instanceof type)) {
//     throw new Error('Value is not an instance of the expected type')
//   }

//   return value
// }

export function fail(message: string): never {
  throw new Error(message)
}

export function assert(assertion: boolean, message?: string): void {
  if (!assertion) {
    throw new Error(`Assertion failed${message !== undefined ? `: ${message}` : ''}`)
  }
}

export function assertNotNull<T>(value: T | null, message?: string): asserts value is T {
  assert(value !== null, message)
}

// TODO: replace all usages with .toSpliced()
export function arrayWithInsertion<T>(arr: T[], index: number, values: T[]): T[] {
  return [...arr.slice(0, index), ...values, ...arr.slice(index)]
}

// TODO: once stabilized, use Map.getOrInsert() inside this function
export function getOrInsertWith<K, V>(map: Map<K, V>, key: K, insertFn: () => V): V {
  let value = map.get(key)

  if (value === undefined) {
    value = insertFn()
    map.set(key, value)
  }

  return value
}

export function filterMap<T, U>(values: T[], filterMapFn: (value: T) => U | null): U[] {
  const result: U[] = []

  for (const value of values) {
    const mapped = filterMapFn(value)

    if (mapped !== null) {
      result.push(mapped)
    }
  }

  return result
}

export type JsonStringifyable =
  | null
  | boolean
  | number
  | string
  | JsonStringifyable[]
  | { [key: string]: JsonStringifyable }
