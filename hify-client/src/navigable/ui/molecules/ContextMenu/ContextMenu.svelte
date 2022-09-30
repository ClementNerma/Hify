<script context="module" lang="ts">
  let _singletonChecker = false
</script>

<script lang="ts">
  import { afterUpdate, onDestroy, onMount } from 'svelte'

  import { getParentNavigable, NavigableItem, RequestFocus } from '../../../navigation'

  import NavigableWithHandlers from '../../../headless/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import ItemStyleLayer from '../../../headless/SimpleNavigableItem/ItemStyleLayer.svelte'
  import Column from '../Column/Column.svelte'
  import { contextMenuStore } from './ContextMenu'
  import SimpleNavigableItem from '../../../headless/SimpleNavigableItem/SimpleNavigableItem.svelte'

  const nav = getParentNavigable()

  let prevFocusItem: NavigableItem<unknown> | null = null

  let ctxTop = -1
  let ctxLeft = -1

  function getBoundingClientRect(el: HTMLElement): DOMRect | null {
    let rect = el.getBoundingClientRect()
    const children = [...el.children]

    while (
      rect.top === 0 &&
      rect.left === 0 &&
      rect.right === 0 &&
      rect.bottom === 0 &&
      rect.height === 0 &&
      rect.width === 0
    ) {
      const child = children.shift()

      if (!child) {
        return null
      }

      rect = child.getBoundingClientRect()
    }

    return rect
  }

  afterUpdate(() => {
    const focusedItem = nav.page.focusedItem()

    if (!$contextMenuStore || !$contextMenuStore.options.length || !focusedItem) {
      return
    }

    const rect = getBoundingClientRect(focusedItem.underlyingElement())

    ctxTop = rect ? (rect.top + rect.bottom) / 2 : 0
    ctxLeft = rect ? (rect.left + rect.right) / 2 : 0

    prevFocusItem = focusedItem

    requestFocus()
  })

  function closeContextMenu() {
    prevFocusItem?.requestFocus()
    contextMenuStore.set(null)
  }

  onMount(() => {
    if (_singletonChecker) {
      throw new Error('Cannot have two ContextMenu elements in the same components tree!')
    }

    _singletonChecker = true
  })

  onDestroy(() => {
    _singletonChecker = false
  })

  let requestFocus: RequestFocus
</script>

{#if $contextMenuStore && $contextMenuStore.options.length > 0}
  <NavigableWithHandlers onBack={closeContextMenu}>
    <div class="container" style="--ctx-top: {ctxTop}px; --ctx-left: {ctxLeft}px;">
      <Column bind:requestFocus>
        {#each $contextMenuStore.options as { label, onPress }}
          <SimpleNavigableItem
            onPress={() => {
              closeContextMenu()
              onPress()
            }}
          >
            <ItemStyleLayer>
              <div class="option">{label}</div>
            </ItemStyleLayer>
          </SimpleNavigableItem>
        {/each}
      </Column>
    </div>
  </NavigableWithHandlers>
{/if}

<style>
  .container {
    position: fixed;
    top: var(--ctx-top);
    left: var(--ctx-left);
    display: inline-block;
    background-color: lightgray;
    color: black;
    border: 1px solid black;
    z-index: 10;
  }

  .option {
    padding: 5px;
  }

  .option:not(:last-child) {
    border-bottom: 1px solid black;
  }
</style>
