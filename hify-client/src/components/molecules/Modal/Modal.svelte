<script context="module" lang="ts">
  export type ModalButton = {
    label: string
    onPress: () => void | boolean | Promise<void | boolean>
  }
</script>

<script lang="ts">
  import Row from '@navigable/ui/molecules/Row/Row.svelte'
  import Button from '@atoms/Button/Button.svelte'
  import { afterUpdate } from 'svelte'
  import { NavigableItem, getParentNavigable } from '@navigable/navigation'

  export let open = false
  export let buttons: ModalButton[]
  export let onOpen: (() => void) | null = null

  const nav = getParentNavigable()

  let prevFocusItem: NavigableItem<unknown> | null = null

  async function onButtonPress(button: ModalButton) {
    if ((await button.onPress()) !== false) {
      open = false
    }
  }

  let wasOpen = open

  afterUpdate(() => {
    if (wasOpen !== open) {
      wasOpen = open

      if (open) {
        prevFocusItem = nav.page.focusedItem()
        onOpen?.()
      } else {
        prevFocusItem?.requestFocus()
      }
    }
  })
</script>

<div class="modal" class:open>
  <div class="modal-inner">
    <slot {open} />

    <Row>
      {#each buttons as button (button.label)}
        <Button onPress={() => onButtonPress(button)}>
          {button.label}
        </Button>
      {/each}
    </Row>
  </div>
</div>

<style>
  .modal {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;

    background-color: rgba(0, 0, 0, 0.5);
  }

  .modal-inner {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .modal:not(.open) {
    display: none;
  }
</style>
