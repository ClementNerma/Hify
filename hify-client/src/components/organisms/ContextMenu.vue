<script setup lang="ts">
import { logFatal } from '@/global/stores/debugger'
import { onUpdated, ref } from 'vue'
import Run from '@/components/atoms/Run.vue'
import { getFocusedItemId, getNavigableDOMElementById, requestFocusById, requestFocusOnElement, type NavigableElementByType } from '@/navigable';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';
import { contextMenuStore } from '@/global/stores/context-menu';
import NavigableList from '@/navigable/vue/components/NavigableList.vue';

function getBoundingClientRect(el: Element): DOMRect | null {
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
  const column = listRef.value ?? logFatal('Column reference not initialized yet')

  const focusedItemId = getFocusedItemId()

  if (!contextMenuStore.value || !contextMenuStore.value.options.length || !focusedItemId) {
    return
  }

  const rect = getBoundingClientRect(getNavigableDOMElementById(focusedItemId) ?? logFatal('Could not get DOM element of currently-focused item'))

  const top = rect ? (rect.top + rect.bottom) / 2 : 0
  const left = rect ? (rect.left + rect.right) / 2 : 0

  ctxTop.value = top + container.clientHeight > window.innerHeight ? window.innerHeight - container.clientHeight - 5 : top
  ctxLeft.value = left + container.clientWidth > window.innerWidth ? window.innerWidth - container.clientWidth - 5 : left

  prevFocusItemId.value = focusedItemId

  requestFocusOnElement(column)
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

const listRef = ref<NavigableElementByType<'list'> | null>(null)
const containerRef = ref<HTMLDivElement | null>(null)
</script>

<template>
  <NavigableList v-if="contextMenuStore && contextMenuStore.options.length > 0" @back-key="closeContextMenu">
    <div
      class="fixed bg-gray-800 text-white border border-solid border-gray-600 z-10 shadow-[2px_2px_5px_rgb(60,60,60)]"
      ref="containerRef" :style="`top: ${ctxTop}px; left: ${ctxLeft}px;`">
      <NavigableList trapped v-slot="{ list }">
        <Run @run="listRef = list" />

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