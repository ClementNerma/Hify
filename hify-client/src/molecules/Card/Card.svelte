<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let _key: string
  export let pictureUrl: string
  export let pictureAlt: string
  export let title: string
  export let onPress: (_key: string) => void
  export let onLongPress: ((_key: string) => void) | undefined = undefined
  export let subtitle: string
</script>

<SimpleNavigableItem onPress={() => onPress(_key)} onLongPress={onLongPress ? () => onLongPress?.(_key) : undefined}>
  <div class="card" on:click={() => onPress(_key)}>
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
