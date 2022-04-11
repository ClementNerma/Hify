<script context="module" lang="ts">
  export type ContextMenuOption = {
    label: string
    onPress: () => void
    // onLongPress?: () => void
  }

  export type ContextMenuStore = Writable<{ options: ContextMenuOption[] } | null>

  export function createContextMenu(): ContextMenuStore {
    return writable(null)
  }

  export function showContextMenu(store: ContextMenuStore, options: ContextMenuOption[]): void {
    store.set({ options })
  }
</script>

<script lang="ts">
  import { afterUpdate } from 'svelte'
  import { Writable, writable } from 'svelte/store'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import { getParentNavigable, RequestFocus } from '../../navigable/navigation'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  const nav = getParentNavigable()

  export let store: ContextMenuStore

  let ctxTop = 100
  let ctxLeft = 100

  afterUpdate(() => {
    if (!$store || !$store.options.length) {
      return
    }

    const focusedItem = nav.page.focusedItem()

    if (!focusedItem) {
      return
    }

    const rect = focusedItem.underlyingElement().getBoundingClientRect()

    ctxTop = (rect.top + rect.bottom) / 2
    ctxLeft = (rect.left + rect.right) / 2

    requestFocus?.()
  })

  export function hideContextMenu(): void {
    store.set(null)
  }

  let requestFocus: RequestFocus | undefined
</script>

{#if $store && $store.options.length > 0}
  <NavigableWithHandlers onBack={hideContextMenu}>
    <div class="container" style="--ctx-top: {ctxTop}px; --ctx-left: {ctxLeft}px;">
      <NavigableList bind:requestFocus>
        {#each $store.options as { label, onPress }}
          <SimpleNavigableItem
            onPress={() => {
              hideContextMenu()
              onPress()
            }}
            transparent={true}
          >
            <div class="option">{label}</div>
          </SimpleNavigableItem>
        {/each}
      </NavigableList>
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
  }

  .option {
    padding: 5px;
  }

  .option:not(:last-child) {
    border-bottom: 1px solid black;
  }
</style>
