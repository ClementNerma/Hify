<script lang="ts">
  export let pictureUrl: string

  export let title: string
  export let subtitle: string

  export let boxSize = 150
  export let enforceMaxWidth = false
  export let rounded = false
</script>

<div class="card {enforceMaxWidth ? 'enforce-max-width' : ''}" style="--width: {boxSize}px">
  <img class="cover {rounded ? 'rounded' : ''}" width={boxSize} height={boxSize} src={pictureUrl} alt="" />
  <div class="title experimental-line-limiter">{title}</div>
  <div class="subtitle experimental-line-limiter">{subtitle}</div>
</div>

<style>
  .card {
    text-align: center;
  }

  .card.enforce-max-width {
    max-width: var(--width);
  }

  .cover.rounded {
    border-radius: 50%;
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
