<script lang="ts">
export type OneListItemChoices<T> = Array<{ id: T, label: string }>

export type OneListSelectExposeType = { buttonRef: ButtonExposeType | null }
</script>

<script setup lang="ts" generic="T extends string">
import NavigableItem, { type NavigableItemExposeType, type NavigableItemProps } from '@/navigable/vue/components/NavigableItem.vue';
import { computed, onBeforeMount, ref } from 'vue';
import { type ButtonExposeType } from '../atoms/Button.vue';
import { logFatal } from '@/global/stores/debugger';
import { NavigationDirection } from '@/navigable';

const props = defineProps<{
  items: OneListItemChoices<T>
  prefix?: string
}>()

defineEmits<{
  press: [T]
  longPress: [T]
}>()

const activeId = defineModel<T>()

const activeIndex = computed(() => (activeId.value && props.items.findIndex(item => item.id === activeId.value)) ?? null)

const expectActiveId = (): T => activeId.value ?? logFatal('Expected a selected item in OneLineList')

function select(itemId: string) {
  const item = props.items.find(item => item.id === itemId) ?? logFatal('Item not foudn in OneLineList')
  activeId.value = item.id
}

const onKeyPress: NavigableItemProps['onKeyPress'] = key => {
  if (activeIndex.value === null || activeIndex.value === -1) {
    return false
  }

  if (key === NavigationDirection.Up && !isFirst.value) {
    select(props.items[activeIndex.value - 1].id)
    return true
  }

  if (key === NavigationDirection.Down && !isLast.value) {
    select(props.items[activeIndex.value + 1].id)
    return true
  }

  return false
}

onBeforeMount(() => {
  if (props.items.length > 0) { select(props.items[0].id) }
})

const isFirst = computed(() => activeIndex.value === null || activeIndex.value === 0)
const isLast = computed(() => activeIndex.value === null || activeIndex.value + 1 === props.items.length)

const itemRef = ref<NavigableItemExposeType | null>(null)

defineExpose({ itemRef })
</script>

<template>
  <NavigableItem ref="itemRef" :on-key-press @press="$emit('press', expectActiveId())"
    @long-press="$emit('longPress', expectActiveId())">
    {{ prefix ?? '' }}
    {{ isFirst && isLast ? '' : isFirst ? '⏷' : isLast ? '⏶' : '⏶⏷' }}
    {{ activeId && items.find(item => item.id === activeId)?.label || '' }}
  </NavigableItem>
</template>

<style scoped>
.option:not(:last-child) {
  border-bottom: 1px solid black;
}
</style>