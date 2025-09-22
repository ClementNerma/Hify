<script setup lang="ts" generic="T extends string">
import { onBeforeMount, onUpdated, ref, watch } from 'vue'
import { logFatal, NavigationDirection, requestFocusOnItem } from '@/navigable'
import NavigableColumn from '@/navigable/vue/components/NavigableColumn.vue'
import NavigableItem, { type NavigableItemExposeType } from '@/navigable/vue/components/NavigableItem.vue'
import Button, { type ButtonExposeType } from '../atoms/Button.vue'

export type DropdownChoices<T> = Array<{ id: T; label: string }>

export type DropdownSelectExposeType = { buttonRef: ButtonExposeType | null }

const props = defineProps<{
  items: DropdownChoices<T>
  defaultId?: T
  prefixLabel?: string
}>()

const selectedId = defineModel<T>()

onBeforeMount(() => {
  selectedId.value = props.defaultId ?? props.items[0]?.id ?? null
})

const opened = ref(false)
const wasOpened = ref(false)

onUpdated(() => {
  if (!opened.value || wasOpened.value) {
    return
  }

  wasOpened.value = true

  const toSelect: T = selectedId.value ?? props.items[0].id

  const { item } = itemsRef.value[toSelect]

  if (!item) {
    logFatal('Missing dropdown element reference')
  }

  requestFocusOnItem(item)
})

function toggle() {
  if (!togglerRef.value || !togglerRef.value.itemRef) {
    logFatal('Unfilled Vue references')
  }

  if (opened.value) {
    opened.value = false
    wasOpened.value = false
    requestFocusOnItem(togglerRef.value.itemRef.item)
  } else {
    opened.value = true
  }
}

function select(id: T) {
  toggle()

  if (selectedId.value !== id) {
    selectedId.value = id
  }
}

function handleRef(ref: NavigableItemExposeType, id: T) {
  itemsRef.value[id] = ref
}

const togglerRef = ref<ButtonExposeType | null>(null)
const itemsRef = ref<Record<string, NavigableItemExposeType>>({})

defineExpose({ buttonRef: togglerRef })
</script>

<template>
  <Button
    @press="toggle()"
    ref="togglerRef"
  >
    {{ prefixLabel ?? '' }}
    <template v-if="selectedId !== null">
      {{items.find(item => item.id === selectedId)?.label ?? ''}}
    </template>
  </Button>

  <div
    class="relative"
    v-if="opened"
  >
    <div class="absolute top-0 border border-solid bg-slate-700">
      <NavigableColumn
        trapped
        ref="menuRef"
        @back-key="toggle()"
        :intercept-key-press="dir => dir === NavigationDirection.Back"
      >
        <NavigableItem
          v-for="item in items"
          :key="item.id"
          @press="select(item.id)"
          :ref="ref => handleRef(ref as any, item.id)"
        >
          <div
            class="choice !px-4 !py-3"
            :class="{ 'bg-slate-600': selectedId === item.id }"
          >
            {{ item.label }}
          </div>
        </NavigableItem>
      </NavigableColumn>
    </div>
  </div>
</template>

<style scoped>
.option:not(:last-child) {
  border-bottom: 1px solid black;
}
</style>