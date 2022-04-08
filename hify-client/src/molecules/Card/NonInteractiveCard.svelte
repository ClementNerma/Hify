<script lang="ts">
  export let pictureUrl: string

  export let title: string
  export let subtitle: string
</script>

<div class="card">
  <img width={150} height={150} src={pictureUrl} alt="" />
  <div class="title experimental-line-limiter">{title}</div>
  <div class="subtitle experimental-line-limiter">{subtitle}</div>
</div>

<style>
  .card {
    text-align: center;
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
