<script lang="ts">
  import { SimpleNavigableItem, SimpleNavigableItemProps } from './SimpleNavigableItem'
  import { getParentNavigable, HTMLNavigableItemWrapperElement } from '../navigation'
  import { onDestroy } from 'svelte'

  export let position: SimpleNavigableItemProps['position'] = null
  export let hasFocusPriority: SimpleNavigableItemProps['hasFocusPriority'] = null

  export let onFocus: SimpleNavigableItemProps['onFocus'] = undefined
  export let onUnfocus: SimpleNavigableItemProps['onUnfocus'] = undefined

  export let onPress: SimpleNavigableItemProps['onPress'] = undefined
  export let onLongPress: SimpleNavigableItemProps['onLongPress'] = undefined
  export let onBack: SimpleNavigableItemProps['onBack'] = undefined

  export let onUp: SimpleNavigableItemProps['onUp'] = undefined
  export let onLeft: SimpleNavigableItemProps['onLeft'] = undefined
  export let onRight: SimpleNavigableItemProps['onRight'] = undefined
  export let onDown: SimpleNavigableItemProps['onDown'] = undefined

  const nav = getParentNavigable()

  const item = new SimpleNavigableItem(nav, {
    position,
    hasFocusPriority,

    onFocus: () => {
      focused = true
      onFocus?.()
    },

    onUnfocus: () => {
      focused = false
      onUnfocus?.()
    },

    onPress,
    onLongPress,
    onBack,

    onUp,
    onLeft,
    onRight,
    onDown,

    getUnderlyingElement: () => {
      if (!wrapper) {
        throw new Error("Tried to access navigable item's underlying wrapper before it is ready")
      }

      return wrapper
    },
  })

  nav.append(item)

  onDestroy(() => {
    nav.remove(item)
  })

  let wrapper: HTMLNavigableItemWrapperElement
  let focused: boolean
</script>

<navigable-item-wrapper
  on:click={() => onPress?.()}
  on:contextmenu|preventDefault={() => onLongPress?.()}
  bind:this={wrapper}
  class:focused
>
  <slot {item} />
</navigable-item-wrapper>

<!-- Removed as "display: contents;" removes the ability to use .scrollIntoView() -->
<style>
  navigable-item-wrapper {
    display: contents;
  }
</style>
