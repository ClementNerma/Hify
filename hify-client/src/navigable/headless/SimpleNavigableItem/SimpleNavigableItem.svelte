<script context="module" lang="ts">
  export type ItemDisplay = 'inline-block' | 'block' | 'transparent' | null
</script>

<script lang="ts">
  import { SimpleNavigableItem } from './SimpleNavigableItem'
  import { getParentNavigable, HTMLNavigableItemWrapperElement, Props } from '../../navigation'
  import { afterUpdate } from 'svelte'

  export let fullHeight = false
  export let noPadding = false
  export let display: ItemDisplay = null
  export let marginRight = 0
  export let notRounded = false
  export let lookalike = false

  export let justForStyle = false

  export let hasFocusPriority: Props<SimpleNavigableItem>['hasFocusPriority'] = null

  export let disabled: Props<SimpleNavigableItem>['disabled'] = undefined

  export let onFocus: Props<SimpleNavigableItem>['onFocus'] = undefined
  export let onUnfocus: Props<SimpleNavigableItem>['onUnfocus'] = undefined

  export let onPress: Props<SimpleNavigableItem>['onPress'] = undefined
  export let onLongPress: Props<SimpleNavigableItem>['onLongPress'] = undefined
  export let onBack: Props<SimpleNavigableItem>['onBack'] = undefined

  export let onUp: Props<SimpleNavigableItem>['onUp'] = undefined
  export let onLeft: Props<SimpleNavigableItem>['onLeft'] = undefined
  export let onRight: Props<SimpleNavigableItem>['onRight'] = undefined
  export let onDown: Props<SimpleNavigableItem>['onDown'] = undefined

  const itemProps = (): Props<SimpleNavigableItem> => ({
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

    getUnderlyingElement: () => wrapper,
  })

  const item = new SimpleNavigableItem(getParentNavigable(true), itemProps())

  afterUpdate(() => item.updateProps(itemProps()))

  export const requestFocus = () => item.requestFocus()

  let wrapper: HTMLNavigableItemWrapperElement
  let focused: boolean
  
  $: translatedDisplay = display === 'transparent' ? 'contents' : display ?? 'inline-block'
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<navigable-item-wrapper
  data-navigable-item-id={!justForStyle ? item.id : ':just_for_style'}
  bind:this={wrapper}
  on:click={() => onPress?.()}
  on:contextmenu|preventDefault={() => onLongPress?.()}
  on:mouseenter={() => item.requestFocus()}
  on:mouseleave={() => item.page.unfocus()}
  class:focused
  class:fullHeight
  class:noPadding
  class:notRounded
  class:lookalike
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
