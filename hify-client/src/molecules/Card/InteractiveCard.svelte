<script lang="ts">
  import { ProgressiveImgFragment } from '../../graphql/generated'
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Card from './Card.svelte'

  export let art: ProgressiveImgFragment | null | undefined

  export let title: string
  export let subtitle: string | null = null

  export let onPress: () => void
  export let onLongPress: (() => void) | undefined = undefined

  export let enforceMaxWidth: boolean | undefined = undefined
  export let rounded: boolean | undefined = undefined
</script>

<SimpleNavigableItem {onPress} {onLongPress}>
  <Card {art} {title} {subtitle} {enforceMaxWidth} {rounded} />
</SimpleNavigableItem>
