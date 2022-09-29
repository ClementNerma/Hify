<script context="module" lang="ts">
  export type ItemDisplay = 'inline-block' | 'block' | 'transparent' | null
</script>

<script lang="ts">
  import { SimpleNavigableItem, SimpleNavigableItemProps } from './SimpleNavigableItem'
  import { getParentNavigable, HTMLNavigableItemWrapperElement } from '../navigation'
  import { afterUpdate, onDestroy } from 'svelte'

  export let fullHeight = false
  export let noPadding = false
  export let display: ItemDisplay = null
  export let marginRight = 0
  export let notRounded = false

  export let position: SimpleNavigableItemProps['position'] = null
  export let hasFocusPriority: SimpleNavigableItemProps['hasFocusPriority'] = null

  export let disabled: SimpleNavigableItemProps['disabled'] = undefined

  export let onFocus: SimpleNavigableItemProps['onFocus'] = undefined
  export let onUnfocus: SimpleNavigableItemProps['onUnfocus'] = undefined

  export let onPress: SimpleNavigableItemProps['onPress'] = undefined
  export let onLongPress: SimpleNavigableItemProps['onLongPress'] = undefined
  export let onBack: SimpleNavigableItemProps['onBack'] = undefined

  export let onUp: SimpleNavigableItemProps['onUp'] = undefined
  export let onLeft: SimpleNavigableItemProps['onLeft'] = undefined
  export let onRight: SimpleNavigableItemProps['onRight'] = undefined
  export let onDown: SimpleNavigableItemProps['onDown'] = undefined

  const itemProps = (): SimpleNavigableItemProps => ({
    position,
    hasFocusPriority,

    disabled,

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

  export const requestFocus = () => item.requestFocus()

  let wrapper: HTMLNavigableItemWrapperElement
  let focused: boolean
  let mouseHover = false

  $: translatedDisplay = display === 'transparent' ? 'contents' : display ?? 'inline-block'
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
  class:fullHeight
  class:noPadding
  class:notRounded
  style="display: {translatedDisplay}; margin-right: {marginRight}px;"
>
  <slot {item} requestFocus={() => item.requestFocus()} />
</navigable-item-wrapper>

<style>
  :global(navigable-item-wrapper:not(.noPadding) > :first-child) {
    padding: 5px;
  }

  navigable-item-wrapper.fullHeight {
    /* -10px for the 2x 5px padding */
    height: calc(100% - 10px);
  }

  navigable-item-wrapper.fullHeight.noPadding {
    height: 100%;
  }

  :global(navigable-item-wrapper.fullHeight > :first-child) {
    min-height: 100%;
  }
</style>
