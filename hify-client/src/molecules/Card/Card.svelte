<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let pictureUrl: string
  export let pictureAlt: string

  export let title: string
  export let subtitle: string

  export let onPress: () => void
  export let onLongPress: (() => void) | undefined = undefined

  export let position: number | null = null
</script>

<SimpleNavigableItem {onPress} {onLongPress} {position}>
  <div class="card">
    <img width={150} height={150} src={pictureUrl} alt={pictureAlt} />
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
