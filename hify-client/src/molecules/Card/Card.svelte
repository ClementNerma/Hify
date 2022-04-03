<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let pictureUrl: string
  export let pictureAlt: string
  export let title: string
  export let onPress: () => void
  export let onLongPress: (() => void) | undefined = undefined
  export let subtitle: string
</script>

<SimpleNavigableItem {onPress} {onLongPress}>
  <div class="card" on:click={onPress}>
    <img width={250} height={250} src={pictureUrl} alt={pictureAlt} />
    <div class="title">{title}</div>
    <div class="subtitle">{subtitle}</div>
  </div>
</SimpleNavigableItem>

<style>
  .card {
    text-align: center;
    padding: 10px;
  }

  .title {
    font-weight: bold;
  }
</style>
