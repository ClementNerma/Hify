<script lang="ts">
  import NavigableRow from '../../../headless/NavigableRow/NavigableRow.svelte'

  export let forCards = false
</script>

<div class="row" class:forCards>
  <NavigableRow>
    <slot />
  </NavigableRow>
</div>

<style>
  .row {
    overflow-x: hidden;
    display: flex;
    flex-direction: row;
  }

  .forCards {
    padding: 100vh 5px !important;
    margin: -100vh 0;
  }
</style>
