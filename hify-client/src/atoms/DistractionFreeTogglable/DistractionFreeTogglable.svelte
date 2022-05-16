<script>
  import { readableDistractionFreeMode } from '../../stores/distraction-free'
</script>

<div class={$readableDistractionFreeMode ? 'hidden' : ''}>
  <slot />
</div>

<style>
  div:not(.hidden) {
    transition: all 0.5s;
  }

  .hidden {
    transition: all 3s;
    opacity: 0;
  }
</style>
