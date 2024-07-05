<script lang="ts">
export type ProgressiveRowExposeType = {
  jumpUnfocusedPosition(newPosition: number): void
  requestFocus(position: number): void

}
</script>

<script setup lang="ts" generic="T extends { [key in K]: string }, K extends string">
import { computed, onUpdated, ref } from 'vue';
import { requestFocusById, type NavigableElementByType } from '@/navigable';
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue';
import NavigableItem, { type NavigableItemExposeType } from '@/navigable/vue/components/NavigableItem.vue';
import { bindRef } from '@/global/utils';

const props = defineProps<{
  items: T[],
  idProp: K,
  disableScroll?: boolean
  initialPosition?: number,
  onItemPress?: (item: T, newPosition: number) => void
  onItemLongPress?: (item: T, newPosition: number) => void
  onFocusChange?: (nowFocused: boolean) => void
}>()

defineSlots<{
  default(props: { item: T, position: number, navigableItem: NavigableElementByType<'item'>, focused: boolean }): unknown
}>()

defineExpose<ProgressiveRowExposeType>({
  jumpUnfocusedPosition(newPosition) {
    position.value = newPosition
    positionOnUnfocused.value = newPosition
  },

  requestFocus(position) {
    requestFocus(position)
  }
})

const position = ref(0)
const disableHandler = ref(false)
const prevSelected = ref<T[K] | null>(null)
const positionOnUnfocused = ref(0)
const isFirstEntering = ref(true)

const COLUMNS = 7

onUpdated(() => {
  if (position.value >= props.items.length) {
    position.value = Math.max(props.items.length - 1, 0)
    requestFocus(position.value)
  } else if (prevSelected.value !== null && !props.items.find((item) => item[props.idProp] === prevSelected.value)) {
    requestFocus(position.value)
  }
})

async function onSelect(newPosition: number, requestItemFocus: boolean) {
  if (disableHandler.value || newPosition < 0) {
    return
  }

  isFirstEntering.value = false

  position.value = Math.min(newPosition, props.items.length - 1)

  if (requestItemFocus) {
    requestFocus(position.value)
  }

  positionOnUnfocused.value = newPosition
}

function requestFocus(position: number) {
  if (props.items.length === 0) {
    return
  }

  const itemId = props.items[position][props.idProp]

  disableHandler.value = true

  const itemRef = (itemsById.value as Record<T[K], NavigableItemExposeType>)[itemId]

  if (itemId) {
    requestFocusById(itemRef.item.id)
  }

  disableHandler.value = false

    ; (prevSelected.value as T[K]) = itemId
}

const firstVisibleItemIndex = computed(() => Math.max(position.value - Math.round((COLUMNS - 1) / 2), 0))
const visibleTracks = computed(() => props.items.slice(firstVisibleItemIndex.value, firstVisibleItemIndex.value + COLUMNS))
const visibleTracksWithPosition = computed(() => visibleTracks.value.map((track, i) => [track, firstVisibleItemIndex.value + i] as const))

const itemsById = ref<Partial<Record<T[K], NavigableItemExposeType>>>({})

const columnSize = computed(() => `${100 / COLUMNS}%`)
</script>

<template>
  <NavigableRow @focus="onFocusChange?.(true)" @unfocus="onFocusChange?.(false)" :disable-scroll>
    <div class="flex flex-row py-2 overflow-hidden w-full">
      <div class="gallery-item" v-for="[item, newPosition], i in visibleTracksWithPosition" :key="item[idProp as K]">
        <NavigableItem :ref="bindRef(itemsById as any, (item as any)[idProp])"
          @left-key="onSelect(newPosition - 1, true)" @right-key="onSelect(newPosition + 1, true)"
          @focus="onSelect(newPosition, false)" @press="onItemPress?.(item, newPosition)"
          @long-press="onItemLongPress?.(item, newPosition)" :has-focus-priority="newPosition === position"
          v-slot="{ item: navigableItem, focused }">

          <slot :item :position="newPosition" :navigableItem :focused />
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
