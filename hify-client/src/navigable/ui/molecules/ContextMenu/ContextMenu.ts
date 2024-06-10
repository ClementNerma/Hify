import type { SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import { ref } from 'vue'

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

	contextMenuStore.value = { options }
}

export const contextMenuStore = ref<ContextMenuData | null>(null)
