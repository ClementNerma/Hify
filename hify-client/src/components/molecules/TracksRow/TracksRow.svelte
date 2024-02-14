<script lang="ts">
  import { ctxMenuCallbacks, ctxMenuOptions } from '@globals/context-menu-items'
  import { AudioTrackFragment } from '@graphql/generated'
  import Card from '@molecules/Card/Card.svelte'
  import ProgressiveRow from '@molecules/ProgressiveRow/ProgressiveRow.svelte'
  import { showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'

  export let tracks: AudioTrackFragment[]
</script>

<ProgressiveRow
  initialItems={tracks}
  idProp="id"
  onItemPress={(track) => ctxMenuCallbacks.playTrack(track, tracks, null)}
  onItemLongPress={(track) =>
    showContextMenu(ctxMenuOptions.forTrack(track, { fromMixId: null }, { context: 'normal' }))}
  let:item={track}
>
  <Card title={track.metadata.tags.title} art={track.metadata.tags.album.art} />
</ProgressiveRow>
