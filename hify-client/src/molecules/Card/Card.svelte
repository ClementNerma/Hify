<script lang="ts">
  import ImgLoader from '../../atoms/ImgLoader/ImgLoader.svelte'
  import { ProgressiveImgFragment } from '../../graphql/generated'

  export let art: ProgressiveImgFragment | null | undefined

  export let title: string
  export let subtitle: string | null = null
  export let focused: boolean

  export let boxSize = 120
  export let circle = false

  // 'enforceMaxWidth' is useful for situations like rows where the card may try to take more width than they should
  // It is harmful though in other contexts like set grids where the width is fixed and cannot be changed
  export let enforceMaxWidth = false
</script>

<div class="card" class:enforceMaxWidth style="--width: {boxSize}px" class:focused>
  <ImgLoader {art} let:src>
    <img class="cover" class:circle width={boxSize} height={boxSize} {src} alt="" />
  </ImgLoader>

  <div class="title experimental-line-limiter">{title}</div>

  {#if subtitle}
    <div class="subtitle experimental-line-limiter">{subtitle}</div>
  {/if}
</div>

<style>
  .card {
    text-align: center;
    transition: transform 0.25s;
  }

  .card.enforceMaxWidth {
    max-width: var(--width);
  }

  .cover.circle {
    border-radius: 50%;
  }

  .card.focused {
    transition: transform 0.25s;
    transform: scale(1.2);
    text-decoration: none !important;
  }

  /* .title {
    font-weight: bold;
  } */

  .subtitle {
    font-size: 0.9rem;
    /* font-style: italic; */
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
