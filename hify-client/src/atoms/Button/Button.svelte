<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let onPress: () => void
  export let disabled = false
</script>

<SimpleNavigableItem transparent {onPress} {disabled}>
  <p class="button {disabled ? 'disabled' : ''}">
    <slot />
  </p>
</SimpleNavigableItem>

<style>
  .button {
    display: inline-block;
    padding: 5px;
    margin: 0;
    border: 1px solid white;
  }

  .button.disabled {
    opacity: 0.5;
  }
</style>
