import { logFatal } from '@/navigable'
import { ref } from 'vue'

export type ContextMenuOption = {
	label: string
	onPress: () => void
	// onLongPress?: () => void
}

export type ContextMenuData = {
	options: ContextMenuOption[]
}

export function showContextMenu(options: ContextMenuOption[]): void {
	if (options.length === 0) {
		logFatal('Cannot create an empty context menu')
	}

	contextMenuStore.value = { options }
}

export const contextMenuStore = ref<ContextMenuData | null>(null)
