<script setup lang="ts" generic="T extends { [key in K]: string }, K extends string">
import { computed, onUpdated, ref } from 'vue';
import Run from '../atoms/Run.vue';
import { requestFocusById, type NavigableElementByType } from '@/navigable';
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';

const props = defineProps<{
  items: T[],
  idProp: K,
  initialPosition?: number,
  onItemPress?: (item: T, newPosition: number) => void
  onItemLongPress?: (item: T, newPosition: number) => void
  onFocusChange?: (nowFocused: boolean) => void
}>()

defineSlots<{
  default(props: { item: T, position: number, navigableItem: NavigableElementByType<'item'>, focused: boolean }): unknown
}>()

defineExpose({
  jumpUnfocusedPosition(newPosition: number) {
    position.value = newPosition
    positionOnUnfocused.value = newPosition
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

  const navItemId = (itemsById.value as Record<T[K], string>)[itemId]

  if (itemId) {
    requestFocusById(navItemId)
  }

  disableHandler.value = false

    ; (prevSelected.value as T[K]) = itemId
}

const firstVisibleItemIndex = computed(() => Math.max(position.value - Math.round((COLUMNS - 1) / 2), 0))
const visibleTracks = computed(() => props.items.slice(firstVisibleItemIndex.value, firstVisibleItemIndex.value + COLUMNS))

const itemsById = ref<Partial<Record<T[K], string>>>({})

const columnSize = computed(() => `${100 / COLUMNS}%`)
</script>

<template>
  <NavigableRow @focus="onFocusChange?.(true)" @unfocus="onFocusChange?.(false)">
    <div class="flex flex-row py-2 overflow-hidden w-full">
      <div class="gallery-item" v-for="item, i in visibleTracks" :key="item[idProp as K]">
        <!-- TODO: const binding newPosition = firstVisibleItemIndex + i -->

        <NavigableItem @left-key="onSelect(firstVisibleItemIndex + i - 1, true)"
          @right-key="onSelect(firstVisibleItemIndex + i + 1, true)" @focus="onSelect(firstVisibleItemIndex + i, false)"
          @press="onItemPress?.(item, firstVisibleItemIndex + i)"
          @long-press="onItemLongPress?.(item, firstVisibleItemIndex + i)"
          :has-focus-priority="firstVisibleItemIndex + i === position" v-slot="{ item: navigableItem, focused }">

          <!-- TODO: simple "ref" binding from Item instead? -->
          <Run @run="(itemsById as any)[(item as any)[idProp]] = navigableItem.id" />

          <slot :item :position="firstVisibleItemIndex + i" :navigableItem :focused />
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
