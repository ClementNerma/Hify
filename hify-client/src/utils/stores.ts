import { useSyncExternalStore } from 'react'

export function createGlobalStore<T>(initialValue: T): GlobalStore<T> {
  let value = initialValue

  const listeners = new Set<(value: T) => void>()

  const mutate = (newValue: T) => {
    if (value === newValue) {
      return
    }

    value = newValue

    for (const listener of listeners) {
      listener(value)
    }
  }

  return {
    subscribe: (callback: (value: T) => void) => {
      listeners.add(callback)

      return () => {
        listeners.delete(callback)
      }
    },

    getSnapshot: () => value,

    mutate,

    mutateWith: (updater: (currentValue: T) => T) => {
      mutate(updater(value))
    },
  }
}

export function useGlobalStore<T>(store: GlobalStore<T>): T {
  return useSyncExternalStore(store.subscribe, store.getSnapshot)
}

export type GlobalStore<T> = {
  subscribe: (callback: (value: T) => void) => () => void
  getSnapshot: () => T
  mutate: (newValue: T) => void
  mutateWith: (updater: (currentValue: T) => T) => void
}
