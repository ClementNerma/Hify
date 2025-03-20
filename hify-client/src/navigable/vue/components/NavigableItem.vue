<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableItem = NavigableCommonElementProps & { type: 'item'; hasFocusPriority?: boolean }

export type NavigableItemProps = {
  disabled?: boolean

  interceptKeyPress?: (
    navigationKey: NavigationDirection | null,
    key: string,
    longPress: boolean,
    modifiers: KeyModifiers,
    item: NavigableItem,
  ) => boolean

  onFocus?: (item: NavigableItem) => void
  onUnfocus?: (item: NavigableItem) => void
  onPress?: (item: NavigableItem) => void
  onLongPress?: (item: NavigableItem) => void
  onLeftKey?: (item: NavigableItem) => void
  onRightKey?: (item: NavigableItem) => void
  onUpKey?: (item: NavigableItem) => void
  onDownKey?: (item: NavigableItem) => void
  onBackKey?: (item: NavigableItem) => void
} & Omit<NavigableItem, 'id' | 'type'>

export type NavigableItemExposeType = {
  item: NavigableItem
  focused: boolean
}
</script>

<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref, type DefineComponent, } from 'vue';
import { NavigationDirection, generateNavigableElementId, navigableElementAttrs, registerNavigableElementHandlers, translateNavigationKey, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type KeyModifiers, type NavigableCommonElementProps, type NavigableElementCustomInteractionHandlers } from '../..';

const props = defineProps<NavigableItemProps>()

const id = generateNavigableElementId()

const item = computed<NavigableItem>(() => ({
  id,
  type: 'item',
  disableScroll: props.disableScroll,
  hasFocusPriority: props.hasFocusPriority,
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

  interceptKeyPress(item, key, longPress, modifiers) {
    const dir = longPress ? null : translateNavigationKey(key)

    if (dir === NavigationDirection.Up) {
      props.onUpKey?.(item)
    } else if (dir === NavigationDirection.Left) {
      props.onLeftKey?.(item)
    } else if (dir === NavigationDirection.Right) {
      props.onRightKey?.(item)
    } else if (dir === NavigationDirection.Down) {
      props.onDownKey?.(item)
    } else if (dir === NavigationDirection.Back) {
      props.onBackKey?.(item)
    }

    return props.interceptKeyPress?.(longPress ? null : dir, key, longPress, modifiers, item) ? { type: 'trap' } : { type: 'native' }
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
