<script lang="ts">
  import { afterUpdate } from 'svelte'

  import { getParentNavigable, NavigableItem, RequestFocus } from '../../navigable/navigation'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { logFatal } from '../../stores/debugger'
  import ItemStyleLayer from '../../navigable/SimpleNavigableItem/ItemStyleLayer.svelte'
  import Column from '../../atoms/Column/Column.svelte'
  import { ContextMenuStore } from './context-menu'

  export let store: ContextMenuStore

  const nav = getParentNavigable()

  let ctxTop = -1
  let ctxLeft = -1
  let prevItem: NavigableItem<unknown> | null = null

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
    if (!$store || !$store.options.length) {
      return
    }

    const focusedItem = nav.page.focusedItem()

    if (!focusedItem) {
      return
    }

    if (!requestFocus) {
      return logFatal('Focus request binding is not defined in ContextMenu component')
    }

    const rect = getBoundingClientRect(focusedItem.underlyingElement())

    ctxTop = rect ? (rect.top + rect.bottom) / 2 : 0
    ctxLeft = rect ? (rect.left + rect.right) / 2 : 0

    prevItem = focusedItem
    requestFocus()
    return
  })

  export function hideContextMenu(): void {
    store.set(null)
    prevItem?.requestFocus()
  }

  let requestFocus: RequestFocus
</script>

{#if $store && $store.options.length > 0}
  <NavigableWithHandlers onBack={hideContextMenu}>
    <div class="container" style="--ctx-top: {ctxTop}px; --ctx-left: {ctxLeft}px;">
      <Column bind:requestFocus>
        {#each $store.options as { label, onPress }}
          <SimpleNavigableItem
            onPress={() => {
              hideContextMenu()
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
