<script lang="ts">
  export let height = 10
</script>

<div class="spacer" style="--height: {height}px" />

<style>
  .spacer {
    height: var(--height);
  }
</style>
