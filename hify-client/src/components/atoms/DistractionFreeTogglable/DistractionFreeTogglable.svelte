<script>
import { distractionFreeMode } from '@stores/distraction-free'
</script>

<div class:hidden={$distractionFreeMode}>
  <slot />
</div>

<style>
  div:not(.hidden) {
    transition: all 0.5s;
  }

  .hidden {
    transition: all 0.7s;
    opacity: 0;
  }
</style>
