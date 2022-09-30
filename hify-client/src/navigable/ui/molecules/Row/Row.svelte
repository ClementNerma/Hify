<script>
  import NavigableRow from '../../../headless/NavigableRow/NavigableRow.svelte'
</script>

<div class="row">
  <NavigableRow>
    <slot />
  </NavigableRow>
</div>

<style>
  .row {
    display: flex;
    flex-direction: row;
    overflow-x: hidden;
  }
</style>
