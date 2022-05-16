<script>
  import { distractionFreeMode } from '../../stores/distraction-free'
</script>

<div class={$distractionFreeMode ? 'hidden' : ''}>
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
