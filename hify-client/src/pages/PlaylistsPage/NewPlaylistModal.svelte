<script lang="ts">
  import { beforeUpdate } from 'svelte'
  import Button from '../../atoms/Button/Button.svelte'
  import NavigableList from '../../navigable/headless/NavigableList/NavigableList.svelte'
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { RequestFocus } from '../../navigable/navigation'

  export let open = false
  export let onSubmit: (name: string) => void
  export let onClose: () => void

  let searchField: HTMLInputElement

  let requestFocus: RequestFocus

  let name = ''

  let prevOpen = open

  beforeUpdate(() => {
    if (open && !prevOpen) {
      requestFocus()
    }

    if (prevOpen !== open) {
      prevOpen = open
    }
  })
</script>

<div id="modal" class:open>
  <NavigableList trapped>
    <SimpleNavigableItem
      onFocus={() => searchField.focus()}
      onUnfocus={() => searchField?.blur()}
      bind:requestFocus
      display="transparent"
      noPadding
      notRounded
    >
      <input type="text" bind:this={searchField} bind:value={name} />
    </SimpleNavigableItem>

    <div>
      <Button
        onPress={() => {
          onClose()
          onSubmit(name)
        }}>Create</Button
      >
      <Button onPress={() => onClose()}>Cancel</Button>
    </div>
  </NavigableList>
</div>

<style>
  #modal {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 50%);
    padding: 25%;
  }

  #modal:not(.open) {
    display: none;
  }
</style>
