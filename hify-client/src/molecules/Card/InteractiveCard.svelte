<script lang="ts">
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Card from './Card.svelte'

  export let pictureUrl: string

  export let title: string
  export let subtitle: string

  export let onPress: () => void
  export let onLongPress: (() => void) | undefined = undefined

  export let enforceMaxWidth: boolean | undefined = undefined
  export let rounded: boolean | undefined = undefined
</script>

<SimpleNavigableItem {onPress} {onLongPress}>
  <Card {pictureUrl} {title} {subtitle} {enforceMaxWidth} {rounded} />
</SimpleNavigableItem>
