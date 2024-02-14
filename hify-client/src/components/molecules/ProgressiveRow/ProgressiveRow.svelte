<script lang="ts" generics="T">
  import { bind } from '@globals/utils'
  import NavigableRow from '@navigable/headless/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { RequestFocus } from '@navigable/navigation'

  export let initialItems: T[]
  export let idProp: keyof T
  export let loadMore: ((count: number) => Promise<T[]>) | null = null

  export let onItemPress: ((item: T) => void) | undefined = undefined
  export let onItemLongPress: ((item: T) => void) | undefined = undefined

  let items = initialItems
  let position = 0

  let handlerDisabled = false

  const COLUMNS = 7

  async function onSelect(newPosition: number) {
    if (handlerDisabled || newPosition < 0) {
      return
    }

    if (newPosition < 0) {
      return
    }

    while (position >= items.length - 2) {
      if (!loadMore) {
        break
      }

      const newItems = await loadMore(COLUMNS)

      if (newItems.length > 0) {
        items = items.concat(newItems)
      } else {
        break
      }
    }

    position = Math.min(newPosition, items.length - 1)

    handlerDisabled = true
    requestFocusById[items[position][idProp]]?.()
    handlerDisabled = false
  }

  $: firstVisibleItemIndex = Math.max(position - Math.round((COLUMNS - 1) / 2), 0)
  $: visibleTracks = items.slice(firstVisibleItemIndex, firstVisibleItemIndex + COLUMNS)

  // @ts-ignore
  let requestFocusById: Record<T[keyof T], RequestFocus> = {}
</script>

<NavigableRow>
  <div class="gallery">
    {#each visibleTracks as item, i (item[idProp])}
      {@const newPosition = firstVisibleItemIndex + i}

      <div class="gallery-item" style="--column-size: {`${100 / COLUMNS}%`}">
        <SimpleNavigableItem
          fullHeight
          onLeft={() => void onSelect(newPosition - 1)}
          onRight={() => void onSelect(newPosition + 1)}
          onFocus={() => void onSelect(newPosition)}
          onPress={() => onItemPress?.(item)}
          onLongPress={() => onItemLongPress?.(item)}
          bind:requestFocus={requestFocusById[item[idProp]]}
          let:item={navigableItem}
          let:focused
        >
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
