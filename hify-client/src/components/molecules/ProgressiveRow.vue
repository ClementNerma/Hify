<script setup lang="ts" generic="T">
import { computed, onUpdated, ref, useTemplateRef } from 'vue'
import {
	getChildrenOf,
	logFatal,
	type NavigableElementByType,
	NavigationDirection,
	requestFocusOnItem,
} from '@/navigable'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'
import NavigableRow, { type NavigableRowExposeType } from '@/navigable/vue/components/NavigableRow.vue'

export type ProgressiveRowExposeType = {
	jumpUnfocusedPosition(newPosition: number): void
	requestFocus(position: number): void
}

const props = defineProps<{
	items: T[]
	disableScroll?: boolean
	initialPosition?: number
	onItemPress?: (item: T, newPosition: number) => void
	onItemLongPress?: (item: T, newPosition: number) => void
	onFocusChange?: (nowFocused: boolean) => void
}>()

defineSlots<{
	default(props: {
		item: T
		position: number
		navigableItem: NavigableElementByType<'item'>
		focused: boolean
	}): unknown
}>()

defineExpose<ProgressiveRowExposeType>({
	jumpUnfocusedPosition(newPosition) {
		// TODO: ignore if row is not focused?
		position.value = newPosition
	},

	requestFocus(position) {
		requestFocus(position)
	},
})

const position = ref(0)
const disableHandler = ref(false)

const COLUMNS = 7

function onFocus(newPosition: number, requestItemFocus: boolean) {
	if (disableHandler.value || newPosition < 0) {
		return
	}

	if (requestItemFocus) {
		requestFocus(Math.min(newPosition, props.items.length - 1))
	}

	position.value = Math.min(newPosition, props.items.length - 1)
}

function requestFocus(position: number) {
	if (!rowRef.value) {
		logFatal('Row reference is not initialized yet in progressive row')
	}

	disableHandler.value = true

	const itemEls = getChildrenOf(rowRef.value.row)
	const itemEl = itemEls[position - computeFirstVisibleItemIndex(position)]

	if (!itemEl) {
		logFatal(`Tried to request focus for position ${position}, but corresponding navigable item was not found`)
	}

	if (itemEl.navEl.type !== 'item') {
		logFatal('Non-item navigable found in progressive row')
	}

	requestFocusOnItem(itemEl.navEl)

	disableHandler.value = false
}

function computeFirstVisibleItemIndex(position: number): number {
	const start = position - Math.round((COLUMNS - 1) / 2)

	if (start < 0) {
		return 0
	}

	if (props.items.length - start + 1 < COLUMNS) {
		return Math.max(props.items.length - COLUMNS + 1, 0)
	}

	return start
}

const firstVisibleItemIndex = computed(() => computeFirstVisibleItemIndex(position.value))
const rowIter = computed(() =>
	props.items
		.slice(firstVisibleItemIndex.value, firstVisibleItemIndex.value + COLUMNS)
		.map((item, i) => ({ itemPosition: firstVisibleItemIndex.value + i, item })),
)

const columnSize = computed(() => `${100 / COLUMNS}%`)

const rowRef = useTemplateRef<NavigableRowExposeType>('rowRef')
</script>

<template>
  <NavigableRow
    @focus="onFocusChange?.(true)"
    @unfocus="onFocusChange?.(false)"
    :disable-scroll
    ref="rowRef"
  >
    <div class="flex flex-row py-2 overflow-hidden w-full">
      <div
        class="gallery-item"
        v-for="{ item, itemPosition }, i in rowIter"
        :key="i"
      >
        <NavigableItem
          :intercept-key-press="d => d === NavigationDirection.Left || d === NavigationDirection.Right"
          @left-key="onFocus(itemPosition - 1, true)"
          @right-key="onFocus(itemPosition + 1, true)"
          @focus="onFocus(itemPosition, false)"
          @press="onItemPress?.(item, itemPosition)"
          @long-press="onItemLongPress?.(item, itemPosition)"
          :has-focus-priority="itemPosition === position"
          v-slot="{ item: navigableItem, focused }"
        >
          <slot
            :item
            :position="itemPosition"
            :navigableItem
            :focused
          />
        </NavigableItem>
      </div>
    </div>
  </NavigableRow>
</template>

<style scoped>
.gallery-item {
  text-align: center;
  min-width: v-bind(columnSize);
  width: v-bind(columnSize);
  max-width: v-bind(columnSize);
}
</style>
