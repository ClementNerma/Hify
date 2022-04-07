<script lang="ts">
  export let pictureUrl: string

  export let title: string
  export let subtitle: string
</script>

<div class="card">
  <img width={150} height={150} src={pictureUrl} alt="" />
  <div class="title">{title}</div>
  <div class="subtitle">{subtitle}</div>
</div>

<style>
  .card {
    text-align: center;
  }

  .title {
    font-weight: bold;
  }
</style>
