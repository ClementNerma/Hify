<script lang="ts">
  import { SimpleNavigableItem, SimpleNavigableItemProps } from './SimpleNavigableItem'
  import { getParentNavigable, HTMLNavigableItemWrapperElement } from '../navigation'
  import { afterUpdate, onDestroy } from 'svelte'
  import { logError } from '../../stores/debugger'

  export let transparent = false
  export let displayBlock = false
  export let style: string | undefined = undefined

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

  if (transparent && displayBlock) {
    logError('Cannot provide both "displayBlock" and "fillHeight" at the same time!')
    transparent = false
  }

  const itemProps = (): SimpleNavigableItemProps => ({
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

  const nav = getParentNavigable(true)

  const item = new SimpleNavigableItem(nav, itemProps())

  nav.append(item)

  afterUpdate(() => {
    item.position = position
    item.hasFocusPriority = hasFocusPriority
    item.props = itemProps()
  })

  onDestroy(() => nav.remove(item))

  let wrapper: HTMLNavigableItemWrapperElement
  let focused: boolean
  let mouseHover = false
</script>

<navigable-item-wrapper
  bind:this={wrapper}
  on:click={() => onPress?.()}
  on:contextmenu|preventDefault={() => onLongPress?.()}
  on:mouseenter={() => {
    mouseHover = true
  }}
  on:mouseleave={() => {
    mouseHover = false
  }}
  class:focusedOrMouseHover={focused || mouseHover}
  class:focused
  class:mouseHover
  class:transparent
  class:displayBlock
  {style}
>
  <slot {item} requestFocus={() => item.requestFocus()} />
</navigable-item-wrapper>

<style>
  .transparent {
    display: contents;
  }

  .displayBlock {
    display: block;
  }
</style>
