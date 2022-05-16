<script>
  import { distractionFreeMode } from '../../stores/distraction-free'
</script>

<div class={$distractionFreeMode ? 'hidden' : ''}>
  <slot />
</div>

<style>
  div {
    transition: all 3s;
  }

  .hidden {
    opacity: 0;
  }
</style>
