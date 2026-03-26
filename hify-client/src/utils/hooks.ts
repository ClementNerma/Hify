import { useEffect, useLayoutEffect, useRef, useState } from 'react'

export function useTimeout(millis: number): boolean {
  const [elapsed, setElapsed] = useState(false)

  useEffect(() => {
    const timeout = setTimeout(() => setElapsed(true), millis)
    return () => clearTimeout(timeout)
  }, [millis])

  return elapsed
}

// Written by Claude Opus 4.5, edited by hand
export function useResettableTimeout(callback: () => void, delay: number): ResettableTimeout {
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const callbackRef = useRef(callback)

  // Keep callback ref up to date
  useEffect(() => {
    callbackRef.current = callback
  }, [callback])

  const clear = ({ runCallback }: { runCallback?: boolean } = {}) => {
    if (timeoutRef.current !== null) {
      clearTimeout(timeoutRef.current)
      timeoutRef.current = null
    }

    if (runCallback === true) {
      callbackRef.current()
    }
  }

  const start = () => {
    clear()

    timeoutRef.current = setTimeout(() => {
      callbackRef.current()
    }, delay)
  }

  // Cleanup on unmount
  useOnUnmounted(clear)

  return { start, restart: start, clear }
}

type ResettableTimeout = {
  start: () => void
  restart: () => void
  clear: (opts?: { runCallback?: boolean }) => void
}

export function useValueWatcher<T extends boolean | number | string | null | undefined>(
  value: T,
  callback: (value: T) => void,
  opts?: { immediate: boolean },
) {
  useValueIdentityWatcher(value, callback, opts)
}

export function useValuesWatcher<T extends (boolean | number | string | null | undefined)[]>(
  values: T,
  callback: (values: T) => void,
  opts?: { immediate: boolean },
) {
  const identity = values.map((v) => v).join('|')
  useValueIdentityWatcher(identity, () => callback(values), opts)
}

export function useValueIdentityWatcher<T>(
  value: T,
  callback: (value: T) => void,
  opts?: { immediate: boolean },
) {
  const [prevValue, setPrevValue] = useState<{ prev: T } | null>(null)

  useOnBeforeMounted(() => {
    if (opts?.immediate === true) {
      setPrevValue({ prev: value })
      callback(value)
    }
  })

  useEffect(() => {
    if (prevValue === null) {
      // oxlint-disable-next-line react-hooks-js/set-state-in-effect: need to track previous value
      setPrevValue({ prev: value })
    } else if (prevValue.prev !== value) {
      setPrevValue({ prev: value })
      callback(value)
    }
  }, [value, prevValue, callback])
}

export function useValueIdentityPrePaintWatcher<T>(value: T, callback: (value: T) => void) {
  const prevValue = useRef<{ prev: T } | null>(null)

  useLayoutEffect(() => {
    // oxlint-disable-next-line typescript/prefer-optional-chain: buggy lint
    if (prevValue.current === null || prevValue.current.prev !== value) {
      prevValue.current = { prev: value }
      callback(value)
    }
  })
}

export function useOnBeforeMounted(callback: () => void): void {
  const triggered = useRef(false)

  useLayoutEffect(() => {
    if (!triggered.current) {
      callback()
      triggered.current = true
    }
  })
}

export function useOnMounted(callback: () => void): void {
  useEffect(
    () => {
      callback()
    },
    // oxlint-disable-next-line eslint-plugin-react-hooks/exhaustive-deps: needs to run once and only once, on mount
    [],
  )
}

export function useOnUnmounted(callback: () => void): void {
  useEffect(
    () => () => {
      callback()
    },
    // oxlint-disable-next-line eslint-plugin-react-hooks/exhaustive-deps: needs to run once and only once, on unmount
    [],
  )
}
