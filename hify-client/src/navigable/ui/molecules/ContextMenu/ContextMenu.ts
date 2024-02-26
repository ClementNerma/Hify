import { SimpleNavigableItemProps } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem'
import { writable } from 'svelte/store'

export type ContextMenuOption = {
	label: string
	onPress: NonNullable<SimpleNavigableItemProps['onPress']>
	// onLongPress?: () => void
}

export type ContextMenuData = {
	options: ContextMenuOption[]
}

export function showContextMenu(options: ContextMenuOption[]): void {
	if (options.length === 0) {
		throw new Error('Cannot create an empty context menu')
	}

	contextMenuStore.set({ options })
}

export const contextMenuStore = writable<ContextMenuData | null>(null)
