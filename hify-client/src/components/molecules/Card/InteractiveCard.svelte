<script lang="ts">
  import { ProgressiveImgFragment } from '@graphql/generated'
  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Card from './Card.svelte'
  import { SimpleNavigableItemProps } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem'

  export let art: ProgressiveImgFragment | null | undefined

  export let title: string
  export let subtitle: string | null = null

  export let onPress: NonNullable<SimpleNavigableItemProps['onPress']>
  export let onLongPress: SimpleNavigableItemProps['onLongPress'] = undefined
  export let onFocus: SimpleNavigableItemProps['onFocus'] = undefined

  export let enforceMaxWidth: boolean | undefined = undefined
  export let circle: boolean | undefined = undefined
</script>

<SimpleNavigableItem {onPress} {onLongPress} {onFocus}>
  <Card {art} {title} {subtitle} {enforceMaxWidth} {circle} />
</SimpleNavigableItem>
