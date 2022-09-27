<script>
  import { readableDistractionFreeMode } from '../../stores/distraction-free'
</script>

<div class:hidden={$readableDistractionFreeMode}>
  <slot />
</div>

<style>
  div:not(.hidden) {
    transition: all 0.5s;
  }

  .hidden {
    transition: all 1s;
    opacity: 0;
  }
</style>
