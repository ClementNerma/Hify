<script lang="ts">
  export let pictureUrl: string
  export let pictureAlt: string
  export let title: string
  export let onClick: () => void
  export let subtitle: string
  export let onSubtitleClick: () => void
</script>

<div class="card" on:click={onClick}>
  <img width={250} height={250} src={pictureUrl} alt={pictureAlt} />
  <div class="title">{title}</div>
  <div class="subtitle" on:click|stopPropagation={onSubtitleClick}>{subtitle}</div>
</div>

<style>
  .card {
    flex: 1 1 0px;
    margin: 0 1%;
  }

  .card:hover {
    cursor: pointer;
  }

  .title {
    font-weight: bold;
  }
</style>
