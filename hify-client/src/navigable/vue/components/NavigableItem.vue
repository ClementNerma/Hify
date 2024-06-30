<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableItem = { id: string, type: 'item', hasFocusPriority?: boolean }

export type NavigableItemProps = {
  disabled?: boolean
  interceptDirectionKeys?: NavigationDirection[]

  display?: CSSProperties['display'],

  onFocus?: (item: NavigableItem) => void,
  onUnfocus?: (item: NavigableItem) => void,
  onPress?: (item: NavigableItem) => void,
  onLongPress?: (item: NavigableItem) => void,
  onDirectionKeyPress?: (key: NavigationDirection, item: NavigableItem) => void,
  onLeftKey?: (item: NavigableItem) => void,
  onRightKey?: (item: NavigableItem) => void,
  onUpKey?: (item: NavigableItem) => void,
  onDownKey?: (item: NavigableItem) => void
  onBackKey?: (item: NavigableItem) => void
} & Omit<NavigableItem, 'id' | 'type'>
</script>

<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref, type CSSProperties, } from 'vue';
import { NavigationDirection, generateNavigableElementId, navigableElementAttrs, registerNavigableElementHandlers, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type NavigableElementCustomInteractionHandlers } from '../..';

const props = defineProps<NavigableItemProps>()

const id = generateNavigableElementId()

const item = computed<NavigableItem>(() => ({
  id,
  type: 'item',
  hasFocusPriority: props.hasFocusPriority
}))

const eventHandlers = computed<NavigableElementCustomInteractionHandlers<'item'>>(() => ({
  press(item) {
    if (!props.disabled) {
      props.onPress?.(item)
    }
  },

  longPress(item) {
    if (!props.disabled) {
      props.onLongPress?.(item)
    }
  },

  directionKeyPress(item, key) {
    if (props.disabled) {
      return { type: 'native' }
    }

    props.onDirectionKeyPress?.(key, item)

    if (key === NavigationDirection.Up) {
      props.onUpKey?.(item)
    } else if (key === NavigationDirection.Left) {
      props.onLeftKey?.(item)
    } else if (key === NavigationDirection.Right) {
      props.onRightKey?.(item)
    } else if (key === NavigationDirection.Down) {
      props.onDownKey?.(item)
    } else if (key === NavigationDirection.Back) {
      props.onBackKey?.(item)
    }

    return props.interceptDirectionKeys?.includes(key) ? { type: 'trap' } : { type: 'native' }
  },

  focus(item) {
    focused.value = true

    if (!props.disabled) {
      props.onFocus?.(item)
    }
  },

  unfocus(item) {
    focused.value = false

    if (!props.disabled) {
      props.onUnfocus?.(item)
    }
  },
}))

onMounted(() => registerNavigableElementHandlers(item.value, eventHandlers.value))
onBeforeUpdate(() => updateNavigableElementHandlers(item.value, eventHandlers.value))
onBeforeUnmount(() => unregisterNavigableElementHandlers(item.value))

const focused = ref(false)

defineExpose({ item, focused })

defineSlots<{
  default(props: { item: NavigableItem, focused: boolean }): unknown
}>()
</script>

<template>
  <navigable-item-wrapper v-bind="navigableElementAttrs(item)">
    <slot :item :focused />
  </navigable-item-wrapper>
</template>

<style scoped>
navigable-item-wrapper {
  display: v-bind(display);
}
</style>
