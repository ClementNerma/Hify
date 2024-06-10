<script setup lang="ts" generic="K extends string, T extends { [key in K]: string }">
import NavigableRow from '@/navigable/headless/NavigableRow/NavigableRow.vue'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import type { SimpleNavigableItem as SimpleNavigableItemClass } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem'
import type { NavigableCommonProps, RequestFocus } from '@/navigable/navigation'
import { computed, onUpdated, ref } from 'vue';
import Run from '../atoms/Run.vue';

const props = defineProps<{
  items: T[],
  idProp: K,
  initialPosition?: number,
  onItemPress?: (item: T, newPosition: number) => void
  onItemLongPress?: (item: T, newPosition: number) => void
  onFocusChange?: NavigableCommonProps['onFocusChange'],
}>()

defineSlots<{
  default(props: { item: T, position: number, navigableItem: SimpleNavigableItemClass, focused: boolean }): unknown
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
  // @ts-expect-error
  itemsById.value[itemId]?.()
  disableHandler.value = false

  // @ts-expect-error
  prevSelected.value = itemId
}

const firstVisibleItemIndex = computed(() => Math.max(position.value - Math.round((COLUMNS - 1) / 2), 0))
const visibleTracks = computed(() => props.items.slice(firstVisibleItemIndex.value, firstVisibleItemIndex.value + COLUMNS))

const itemsById = ref<Partial<Record<T[K], RequestFocus>>>({})
</script>

<template>
  <NavigableRow :on-focus-change>
    <div class="flex flex-row py-2 overflow-hidden">
      <div class="gallery-item" v-for="item, i in visibleTracks" :key="item[idProp as K]"
        :style="`--column-size: ${100 / COLUMNS}%`">
        <!-- TODO: const binding newPosition = firstVisibleItemIndex + i -->

        <SimpleNavigableItem @left="onSelect(firstVisibleItemIndex + i - 1, true)"
          @right="onSelect(firstVisibleItemIndex + i + 1, true)" @focus="onSelect(firstVisibleItemIndex + i, false)"
          @press="onItemPress?.(item, firstVisibleItemIndex + i)"
          @long-press="onItemLongPress?.(item, firstVisibleItemIndex + i)" v-slot="{ item: navigableItem, focused }"
          :has-focus-priority="firstVisibleItemIndex + i === position">

          <Run @run="
            // @ts-expect-error
            itemsById[item[idProp]] = () => navigableItem.requestFocus()
            " />

          <slot :item :position="firstVisibleItemIndex + i" :navigableItem :focused />
        </SimpleNavigableItem>
      </div>
    </div>
  </NavigableRow>
</template>

<style scoped>
.gallery-item {
  text-align: center;
  min-width: var(--column-size);
  width: var(--column-size);
  max-width: var(--column-size);
}
</style>
