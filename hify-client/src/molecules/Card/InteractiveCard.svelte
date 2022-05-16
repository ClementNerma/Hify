<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Card from './Card.svelte'

  export let pictureUrl: string

  export let title: string
  export let subtitle: string

  export let onPress: () => void
  export let onLongPress: (() => void) | undefined = undefined

  export let position: number | null = null

  export let enforceMaxWidth: boolean | undefined = undefined
</script>

<SimpleNavigableItem {onPress} {onLongPress} {position}>
  <Card {pictureUrl} {title} {subtitle} {enforceMaxWidth} />
</SimpleNavigableItem>
