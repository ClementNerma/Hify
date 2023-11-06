<script lang="ts">
  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { RequestFocus } from '@navigable/navigation'

  let searchField: HTMLInputElement

  export let requestFocus: RequestFocus

  export let value: string
</script>

<SimpleNavigableItem
  onFocus={() => searchField.focus()}
  onUnfocus={() => searchField.blur()}
  bind:requestFocus
  display="transparent"
  noPadding
  notRounded
>
  <input type="text" bind:this={searchField} bind:value />
</SimpleNavigableItem>
