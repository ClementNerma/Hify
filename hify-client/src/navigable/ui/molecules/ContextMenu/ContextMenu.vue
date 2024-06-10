<script setup lang="ts">
import type { NavigableItem } from '@/navigable/navigation'
import NavigableWithHandlers from '@/navigable/headless/NavigableWithHandlers/NavigableWithHandlers.vue'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import Column from '../Column/Column.vue'
import { contextMenuStore } from './ContextMenu'
import { logFatal } from '@/global/stores/debugger'
import { onUpdated, ref } from 'vue'
import Run from '@/components/atoms/Run.vue'
import type { NavigableList } from '@/navigable/headless/NavigableList/NavigableList'

function getBoundingClientRect(el: HTMLElement): DOMRect | null {
  let rect = el.getBoundingClientRect()
  const children = Array.from(el.children)

  while (
    rect.top === 0 &&
    rect.left === 0 &&
    rect.right === 0 &&
    rect.bottom === 0 &&
    rect.height === 0 &&
    rect.width === 0
  ) {
    const child = children.shift()

    if (!child) {
      return null
    }

    rect = child.getBoundingClientRect()
  }

  return rect
}

onUpdated(() => {
  if (!containerRef.value) {
    return
  }

  const container = containerRef.value
  const column = columnRef.value ?? logFatal('Column reference not initialized yet')

  const focusedItem = column.page.focusedItem()
  if (!contextMenuStore.value || !contextMenuStore.value.options.length || !focusedItem) {
    return
  }

  const rect = getBoundingClientRect(focusedItem.underlyingElement())

  const top = rect ? (rect.top + rect.bottom) / 2 : 0
  const left = rect ? (rect.left + rect.right) / 2 : 0

  ctxTop.value = top + container.clientHeight > window.innerHeight ? window.innerHeight - container.clientHeight - 5 : top
  ctxLeft.value = left + container.clientWidth > window.innerWidth ? window.innerWidth - container.clientWidth - 5 : left

  prevFocusItem.value = focusedItem

  column.requestFocus()
})

function closeContextMenu() {
  prevFocusItem.value?.requestFocus()
  contextMenuStore.value = null
}

const prevFocusItem = ref<NavigableItem<unknown> | null>(null)

const ctxTop = ref(-1)
const ctxLeft = ref(-1)

const columnRef = ref<NavigableList | null>(null)
const containerRef = ref<HTMLDivElement | null>(null)
</script>

<template>
  <NavigableWithHandlers v-if="contextMenuStore && contextMenuStore.options.length > 0" @back="closeContextMenu">
    <div class="container" ref="containerRef" :style="`top: ${ctxTop}px; left: ${ctxLeft}px;`">
      <Column trapped v-slot="{ column }">
        <Run :run="() => columnRef = column" />

        <SimpleNavigableItem v-for="option in contextMenuStore.options" :key="option.label"
          @press="closeContextMenu(); option.onPress()" v-slot="{ focused }">
          <div class="option-container" :class="{ focused }">
            <div class="option">{{ option.label }}</div>
          </div>
        </SimpleNavigableItem>
      </Column>
    </div>
  </NavigableWithHandlers>
</template>

<style scoped>
.container {
  position: fixed;

  background-color: rgb(42, 42, 42);
  color: white;

  border: 1px solid rgb(78, 78, 78);
  border-radius: 5px;

  box-shadow: 2px 2px 5px rgb(60, 60, 60);

  z-index: 10;
}

.option-container.focused {
  background-color: darkgray;
}

.option {
  padding: 5px;
}

.option:not(:last-child) {
  border-bottom: 1px solid black;
}
</style>