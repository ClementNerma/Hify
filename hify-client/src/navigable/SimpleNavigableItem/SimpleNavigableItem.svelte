<script lang="ts">
  import { SimpleNavigableItem, SimpleNavigableItemProps } from './SimpleNavigableItem'
  import { getParentNavigable, HTMLNavigableItemWrapperElement } from '../navigation'
  import { onDestroy } from 'svelte'

  export let onPress: SimpleNavigableItemProps['onPress'] = undefined
  export let onLongPress: SimpleNavigableItemProps['onLongPress'] = undefined
  export let onFocusChange: SimpleNavigableItemProps['onFocusChange'] = undefined
  export let onBack: SimpleNavigableItemProps['onBack'] = undefined

  const nav = getParentNavigable()

  const item = new SimpleNavigableItem(nav, {
    onPress,
    onLongPress,
    onFocusChange: (has) => {
      focused = has
      onFocusChange?.(has)
    },
    onBack,
    getUnderlyingElement: () => {
      if (!wrapper) {
        throw new Error("Tried to access navigable item's underlying wrapper before it is ready")
      }

      return wrapper
    },
  })

  nav.append(item)

  onDestroy(() => nav.remove(item))

  let wrapper: HTMLNavigableItemWrapperElement
  let focused: boolean
</script>

<navigable-item-wrapper
  on:click={() => onPress?.()}
  on:contextmenu|preventDefault={() => onLongPress?.()}
  bind:this={wrapper}
  class:focused
>
  <slot />
</navigable-item-wrapper>

<style>
  navigable-item-wrapper {
    display: contents;
  }
</style>
