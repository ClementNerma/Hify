<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let onPress: () => void
  export let disabled = false
</script>

<SimpleNavigableItem transparent {onPress} {disabled}>
  <span class="button {disabled ? 'disabled' : ''}">
    <slot />
  </span>
</SimpleNavigableItem>

<style>
  .button {
    display: inline-block;
    width: fit-content;
    padding: 5px;
    margin: 5px;
    border: 1px solid white;
  }

  .button.disabled {
    opacity: 0.5;
  }
</style>
