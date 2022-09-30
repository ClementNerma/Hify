<script lang="ts">
  export let pictureUrl: string

  export let title: string
  export let subtitle: string | null

  export let boxSize = 120
  export let rounded = false

  // 'enforceMaxWidth' is useful for situations like rows where the card may try to take more width than they should
  // It is harmful though in other contexts like set grids where the width is fixed and cannot be changed
  export let enforceMaxWidth = false
</script>

<div class="card" class:enforceMaxWidth style="--width: {boxSize}px">
  <img class="cover" class:rounded width={boxSize} height={boxSize} src={pictureUrl} alt="" />
  <div class="title experimental-line-limiter">{title}</div>

  {#if subtitle}
    <div class="subtitle experimental-line-limiter">{subtitle}</div>
  {/if}
</div>

<style>
  .card {
    text-align: center;
  }

  .card.enforceMaxWidth {
    max-width: var(--width);
  }

  .cover.rounded {
    border-radius: 50%;
  }

  .title {
    font-weight: bold;
  }

  .subtitle {
    font-size: 0.9rem;
  }

  /* TODO: remove experimental stuff */
  .experimental-line-limiter {
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
  }
</style>
