<script context="module" lang="ts">
  let _singletonChecker = false
</script>

<script lang="ts">
  import { afterUpdate, onDestroy, onMount } from 'svelte'

  import { getParentNavigable, NavigableItem, RequestFocus } from '../../../navigation'

  import NavigableWithHandlers from '../../../headless/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import Column from '../Column/Column.svelte'
  import { contextMenuStore } from './ContextMenu'
  import SimpleNavigableItem from '../../../headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Setter, { setterStore } from '../../../../atoms/Setter/Setter.svelte'
  import { get } from 'svelte/store'
  import { logFatal } from '../../../../stores/debugger'

  const nav = getParentNavigable()

  let prevFocusItem: NavigableItem<unknown> | null = null

  let ctxTop = -1
  let ctxLeft = -1

  function getBoundingClientRect(el: HTMLElement): DOMRect | null {
    let rect = el.getBoundingClientRect()
    const children = Array.from(el.children)

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

    const top = rect ? (rect.top + rect.bottom) / 2 : 0
    const left = rect ? (rect.left + rect.right) / 2 : 0

    ctxTop = top + ctxMenuHeight > window.innerHeight ? window.innerHeight - ctxMenuHeight - 5 : top
    ctxLeft = left + ctxMenuWidth > window.innerWidth ? window.innerWidth - ctxMenuWidth - 5 : left

    prevFocusItem = focusedItem

    const requestFocus = get(_requestFocus) ?? logFatal('Focus request handler is not available')
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

  let _requestFocus = setterStore<RequestFocus>()

  let ctxMenuWidth: number
  let ctxMenuHeight: number
</script>

{#if $contextMenuStore && $contextMenuStore.options.length > 0}
  <NavigableWithHandlers onBack={closeContextMenu}>
    <div
      class="container"
      style="--ctx-top: {ctxTop}px; --ctx-left: {ctxLeft}px;"
      bind:clientWidth={ctxMenuWidth}
      bind:clientHeight={ctxMenuHeight}
    >
      <!-- Multi-level bindings are not supported so we use a basic callback system instead -->
      <Column trapped let:requestFocus>
        <Setter value={requestFocus} writeTo={_requestFocus} />
        {#each $contextMenuStore.options as { label, onPress }}
          <SimpleNavigableItem
            let:focused
            onPress={() => {
              closeContextMenu()
              onPress()
            }}
          >
            <div class="option-container" class:focused>
              <div class="option">{label}</div>
            </div>
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

    background-color: rgb(42, 42, 42);
    color: white;

    border: 1px solid rgb(78, 78, 78);
    border-radius: 5px;

    box-shadow: 2px 2px 5px rgb(60, 60, 60);

    z-index: 10;
  }

  .option-container.focused {
    background-color: darkgray;
  }

  .option {
    padding: 5px;
  }

  .option:not(:last-child) {
    border-bottom: 1px solid black;
  }
</style>
