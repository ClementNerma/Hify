import { useSyncExternalStore } from 'react'

/**
 * Create a global store, which can be subscribed to from both
 * within a React component and outside of React components.
 *
 * This is useful for global state management, such as for a player or a context menu.
 *
 * @template T The type of the value stored in the global store.
 * @param initialValue - The initial value of the store.
 *
 * @returns An object representing the global store, with methods to subscribe, get the current value, and mutate the value.
 */
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

/**
 * Use a global store within a React component, subscribing to its updates and re-rendering the component when the store's value changes.
 *
 * @template T The type of the value stored in the global store.
 * @param store - The global store to subscribe to.
 *
 * @returns The current value of the global store.
 */
export function useGlobalStore<T>(store: GlobalStore<T>): T {
  return useSyncExternalStore(store.subscribe, store.getSnapshot)
}

export type GlobalStore<T> = {
  subscribe: (callback: (value: T) => void) => () => void
  getSnapshot: () => T
  mutate: (newValue: T) => void
  mutateWith: (updater: (currentValue: T) => T) => void
}
