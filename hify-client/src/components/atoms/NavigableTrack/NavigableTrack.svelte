<script context="module" lang="ts">
</script>

<script lang="ts">
  import { AudioTrackFragment } from '@graphql/generated'
  import { showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'

  import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ItemDisplay } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { SimpleNavigableItemProps } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem'
  import { EntryInPlaylist, ctxMenuCallbacks, ctxMenuOptions } from '@globals/context-menu-items'

  export let tracks: AudioTrackFragment[]
  export let track: AudioTrackFragment
  export let inPlaylist: EntryInPlaylist | null = null
  export let goToAlbumOption = true
  export let display: ItemDisplay = null
  export let fromMixId: string | null = null
  export let onFocus: SimpleNavigableItemProps['onFocus'] = undefined
</script>

<SimpleNavigableItem
  let:item
  let:focused
  onPress={() => ctxMenuCallbacks.playTrack(track, tracks, fromMixId)}
  onLongPress={() => showContextMenu(ctxMenuOptions.forTrack(track, { fromMixId, goToAlbumOption, inPlaylist }))}
  fullHeight
  {display}
  {onFocus}
>
  <slot {item} {focused} />
</SimpleNavigableItem>
