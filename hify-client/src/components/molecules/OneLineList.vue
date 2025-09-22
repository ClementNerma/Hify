<script setup lang="ts" generic="T extends string">
import { computed, onBeforeMount, onBeforeUpdate, onUpdated, ref } from 'vue'
import { logFatal, NavigationDirection } from '@/navigable'
import NavigableItem, {
  type NavigableItemExposeType,
  type NavigableItemProps,
} from '@/navigable/vue/components/NavigableItem.vue'
import { type ButtonExposeType } from '../atoms/Button.vue'

export type OneListItemChoices<T> = Array<{ id: T; label: string }>

export type OneListSelectExposeType = { buttonRef: ButtonExposeType | null }

const props = defineProps<{
  items: OneListItemChoices<T>
  prefix?: string
}>()

defineEmits<{
  press: [T]
  longPress: [T]
}>()

const activeId = defineModel<T | null>()

const activeIndex = computed(() => {
  const active = props.items.findIndex((item) => item.id === activeId.value)
  return active !== -1 ? active : null
})

const expectActiveId = (): T => activeId.value ?? logFatal('Expected a selected item in OneLineList')

const interceptKeyPress: NavigableItemProps['interceptKeyPress'] = (dir) => {
  if (activeIndex.value === null || activeIndex.value === -1) {
    return false
  }

  if (dir === NavigationDirection.Up && !isFirst.value) {
    activeId.value = props.items[activeIndex.value - 1].id
    return true
  }

  if (dir === NavigationDirection.Down && !isLast.value) {
    activeId.value = props.items[activeIndex.value + 1].id
    return true
  }

  return false
}

onBeforeMount(() => {
  if (props.items.length > 0) {
    activeId.value = props.items[0].id
  }
})

onBeforeUpdate(() => {
  if (activeId.value !== null && activeIndex.value === null) {
    activeId.value = props.items.length > 0 ? props.items[0].id : null
  }
})

const isFirst = computed(() => activeIndex.value === null || activeIndex.value === 0)
const isLast = computed(() => activeIndex.value === null || activeIndex.value + 1 === props.items.length)

const itemRef = ref<NavigableItemExposeType | null>(null)

defineExpose({ itemRef })
</script>

<template>
  <NavigableItem
    ref="itemRef"
    :intercept-key-press
    @press="$emit('press', expectActiveId())"
    @long-press="$emit('longPress', expectActiveId())"
  >
    {{ prefix ?? '' }}
    {{ isFirst && isLast ? '' : isFirst ? '⏷' : isLast ? '⏶' : '⏶⏷' }}
    {{activeId && items.find(item => item.id === activeId)?.label || ''}}
  </NavigableItem>
</template>

<style scoped>
.option:not(:last-child) {
  border-bottom: 1px solid black;
}
</style>