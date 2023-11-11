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
  import { NavigableItem, RequestFocus, getParentNavigable } from '@navigable/navigation'
  import NavigableList from '@navigable/headless/NavigableList/NavigableList.svelte'

  export let open = false
  export let buttons: ModalButton[]
  export let onOpen: (() => void) | null = null

  const nav = getParentNavigable()

  let prevFocusItem: NavigableItem<unknown> | null = null

  let loading = false

  async function onButtonPress(button: ModalButton) {
    if (loading) {
      return
    }

    loading = true

    if ((await button.onPress()) !== false) {
      open = false
    }

    loading = false
  }

  let wasOpen = false

  afterUpdate(() => {
    if (wasOpen !== open) {
      wasOpen = open

      if (open) {
        prevFocusItem = nav.page.focusedItem()
        buttonsRequestFocus[0]?.()
        onOpen?.()
      } else {
        prevFocusItem?.requestFocus()
      }
    }
  })

  const buttonsRequestFocus: RequestFocus[] = new Array(buttons.length)
</script>

<div class="modal" class:open>
  <div class="modal-inner">
    <NavigableList trapped>
      <slot {open} />

      <div class="buttons">
        <Row>
          {#each buttons as button, i (button.label)}
            <Button bind:requestFocus={buttonsRequestFocus[i]} onPress={() => onButtonPress(button)} let:focused>
              {#if loading && focused}
                <em>Loading...</em>
              {:else}
                {button.label}
              {/if}
            </Button>
          {/each}
        </Row>
      </div>
    </NavigableList>
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

    padding: 10px;
    font-size: 15px;
    border-radius: 5px;
    background-color: lightgray;
    color: black;
  }

  .buttons {
    margin-top: 15px;
  }

  .modal:not(.open) {
    display: none;
  }
</style>
