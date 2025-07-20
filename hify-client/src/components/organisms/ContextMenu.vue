<script setup lang="ts">
import { onUpdated, ref, watch } from 'vue'
import { contextMenuStore } from '@/global/stores/context-menu'
import {
	getFocusedItemId,
	getNavigableDOMElementById,
	logFatal,
	type NavigableElementByType,
	NavigationDirection,
	requestFocusById,
	requestFocusOnElement,
} from '@/navigable'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'
import NavigableList, { type NavigableListExposeType } from '@/navigable/vue/components/NavigableList.vue'

onUpdated(() => {
	if (!containerRef.value) {
		return
	}

	const ctxMenuContainer = containerRef.value
	const column = listRef.value ?? logFatal('Column reference not initialized yet')

	const focusedItemId = getFocusedItemId()

	if (!contextMenuStore.value || !contextMenuStore.value.options.length || !focusedItemId) {
		return
	}

	const focusedDomEl = getNavigableDOMElementById(focusedItemId) ?? logFatal('Focused DOM element not found')
	const rect = focusedDomEl.getBoundingClientRect()

	const top = rect ? (rect.top + rect.bottom) / 2 : 0
	const left = rect ? rect.left + Math.min(30, rect.width / 2) : 0

	ctxTop.value =
		top + ctxMenuContainer.clientHeight > window.innerHeight
			? window.innerHeight - ctxMenuContainer.clientHeight - 5
			: top
	ctxLeft.value =
		left + ctxMenuContainer.clientWidth > window.innerWidth
			? window.innerWidth - ctxMenuContainer.clientWidth - 5
			: left

	prevFocusItemId.value = focusedItemId

	requestFocusOnElement(column.list)
})

function closeContextMenu() {
	if (prevFocusItemId.value) {
		requestFocusById(prevFocusItemId.value)
		prevFocusItemId.value = null
	}

	contextMenuStore.value = null
}

const prevFocusItemId = ref<string | null>(null)

const ctxTop = ref(-1)
const ctxLeft = ref(-1)

const listRef = ref<NavigableListExposeType | null>(null)
const containerRef = ref<HTMLDivElement | null>(null)
</script>

<template>
	<NavigableList v-if="contextMenuStore && contextMenuStore.options.length > 0"
		:intercept-key-press="dir => dir === NavigationDirection.Back" @back-key="closeContextMenu">
		<div class="ctxmenu fixed bg-gray-800 text-white border border-solid border-gray-600 z-10 shadow-[2px_2px_5px_rgb(60,60,60)]"
			ref="containerRef" :style="`top: ${ctxTop}px; left: ${ctxLeft}px;`">
			<!-- TODO: implement "trapped" -->
			<NavigableList trapped ref="listRef">
				<NavigableItem v-for="option in contextMenuStore.options" :key="option.label"
					@press="closeContextMenu(); option.onPress()" v-slot="{ focused }">
					<div :class="{ 'bg-gray-400': focused }">
						<div class="p-1.5 option">{{ option.label }}</div>
					</div>
				</NavigableItem>
			</NavigableList>
		</div>
	</NavigableList>
</template>

<style scoped>
.option:not(:last-child) {
	border-bottom: 1px solid black;
}
</style>