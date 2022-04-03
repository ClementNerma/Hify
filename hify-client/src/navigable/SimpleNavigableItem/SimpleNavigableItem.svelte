<script lang="ts">
  import { SimpleNavigableItem, SimpleNavigableItemProps } from './SimpleNavigableItem'
  import { getParentNavigable, HTMLItemWrapperElement } from '../navigation'
  import { onDestroy } from 'svelte'

  export let onPress: SimpleNavigableItemProps['onPress'] = undefined
  export let onLongPress: SimpleNavigableItemProps['onLongPress'] = undefined
  export let onFocusChange: SimpleNavigableItemProps['onFocusChange'] = undefined
  export let onBack: SimpleNavigableItemProps['onBack'] = undefined

  const nav = getParentNavigable()

  const item = new SimpleNavigableItem(nav, {
    onPress,
    onLongPress,
    onFocusChange,
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

  let wrapper: HTMLItemWrapperElement
</script>

<navigable-item-wrapper bind:this={wrapper}>
  <slot />
</navigable-item-wrapper>
