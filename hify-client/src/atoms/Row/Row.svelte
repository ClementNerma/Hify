<script>
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
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
