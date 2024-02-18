<script lang="ts" generics="T">
  import { afterUpdate } from 'svelte'

  import NavigableRow from '@navigable/headless/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { NavigableCommonProps, RequestFocus } from '@navigable/navigation'

  export let items: T[]
  export let idProp: keyof T

  export let onItemPress: ((item: T, newPosition: number) => void) | undefined = undefined
  export let onItemLongPress: ((item: T, newPosition: number) => void) | undefined = undefined

  export let onFocusChange: NavigableCommonProps['onFocusChange'] = undefined

  let position = 0
  let handlerDisabled = false
  let prevSelected: T[keyof T] | null = null

  let positionOnUnfocused = 0

  const COLUMNS = 7

  afterUpdate(() => {
    if (position >= items.length) {
      position = Math.max(items.length - 1, 0)
      requestFocus(position)
    } else if (prevSelected !== null && !items.find((item) => item[idProp] === prevSelected)) {
      requestFocus(position)
    }
  })

  async function onSelect(newPosition: number, requestItemFocus: boolean) {
    if (handlerDisabled || newPosition < 0) {
      return
    }

    position = Math.min(newPosition, items.length - 1)

    if (requestItemFocus) {
      requestFocus(position)
    }

    positionOnUnfocused = newPosition
  }

  function requestFocus(position: number) {
    if (items.length === 0) {
      return
    }

    handlerDisabled = true
    requestFocusById[items[position][idProp]]?.()
    handlerDisabled = false
    prevSelected = items[position][idProp]
  }

  $: firstVisibleItemIndex = Math.max(position - Math.round((COLUMNS - 1) / 2), 0)
  $: visibleTracks = items.slice(firstVisibleItemIndex, firstVisibleItemIndex + COLUMNS)

  // @ts-ignore
  let requestFocusById: Record<T[keyof T], RequestFocus> = {}
</script>

<NavigableRow {onFocusChange}>
  <div class="gallery">
    {#each visibleTracks as item, i (item[idProp])}
      {@const newPosition = firstVisibleItemIndex + i}

      <div class="gallery-item" style="--column-size: {`${100 / COLUMNS}%`}">
        <SimpleNavigableItem
          fullHeight
          onLeft={() => void onSelect(newPosition - 1, true)}
          onRight={() => void onSelect(newPosition + 1, true)}
          onFocus={() => void onSelect(newPosition, false)}
          onPress={() => onItemPress?.(item, newPosition)}
          onLongPress={() => onItemLongPress?.(item, newPosition)}
          hasFocusPriority={newPosition === positionOnUnfocused}
          bind:requestFocus={requestFocusById[item[idProp]]}
          let:item={navigableItem}
          let:focused
        >
          {newPosition} / {positionOnUnfocused}
          <slot {item} position={newPosition} {navigableItem} {focused} />
        </SimpleNavigableItem>
      </div>
    {/each}
  </div>
</NavigableRow>

<style>
  .gallery {
    padding: 7px 0;
    display: flex;
    flex-direction: row;
    overflow: hidden;
  }

  .gallery-item {
    text-align: center;
    min-width: var(--column-size);
    width: var(--column-size);
    max-width: var(--column-size);
  }
</style>
