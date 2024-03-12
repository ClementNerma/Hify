<script lang="ts">
export let inlineBlock = false
</script>

<div class:inlineBlock><slot /></div>

<style>
  .inlineBlock {
    display: inline-block;
  }
</style>
