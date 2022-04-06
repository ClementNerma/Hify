<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import NonInteractiveCard from './NonInteractiveCard.svelte'

  export let pictureUrl: string
  export let pictureAlt: string

  export let title: string
  export let subtitle: string

  export let onPress: () => void
  export let onLongPress: (() => void) | undefined = undefined

  export let position: (() => number) | null = null
</script>

<SimpleNavigableItem {onPress} {onLongPress} {position}>
  <NonInteractiveCard {pictureUrl} {pictureAlt} {title} {subtitle} />
</SimpleNavigableItem>
