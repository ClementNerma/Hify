<script lang="ts">
  export let pictureUrl: string

  export let title: string
  export let subtitle: string

  export let boxSize = 150
</script>

<div class="card" style="--width: {boxSize}px">
  <img width={boxSize} height={boxSize} src={pictureUrl} alt="" />
  <div class="title experimental-line-limiter">{title}</div>
  <div class="subtitle experimental-line-limiter">{subtitle}</div>
</div>

<style>
  .card {
    text-align: center;
    max-width: var(--width);
  }

  .title {
    font-weight: bold;
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
