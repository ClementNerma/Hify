import { writable, Writable } from 'svelte/store'

export type ContextMenuOption = {
  label: string
  onPress: () => void
  // onLongPress?: () => void
}

export type ContextMenuStore = Writable<{ options: ContextMenuOption[] } | null>

export function createContextMenu(): ContextMenuStore {
  return writable(null)
}

export function showContextMenu(store: ContextMenuStore, options: ContextMenuOption[]): void {
  store.set({ options })
}
