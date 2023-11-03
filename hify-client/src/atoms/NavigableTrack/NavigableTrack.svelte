<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AudioTrackFragment, MixParams } from '../../graphql/generated'
  import { showContextMenu } from '../../navigable/ui/molecules/ContextMenu/ContextMenu'

  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ItemDisplay } from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ROUTES } from '../../routes'
  import { enqueue, playTrackFromNewQueue } from '../../stores/play-queue'

  export let tracks: AudioTrackFragment[]
  export let track: AudioTrackFragment
  export let goToAlbumOption = true
  export let display: ItemDisplay = null
  export let fromMixParams: MixParams | null = null

  function play() {
    playTrackFromNewQueue(tracks, tracks.indexOf(track), fromMixParams)
    navigate(ROUTES.nowPlaying)
  }
</script>

<SimpleNavigableItem
  let:item
  let:focused
  onPress={play}
  onLongPress={() =>
    showContextMenu(
      (goToAlbumOption
        ? [{ label: 'Go to album', onPress: () => navigate(ROUTES.album(track.metadata.tags.album.id)) }]
        : []
      ).concat([
        { label: 'Play next', onPress: () => enqueue([track], 'next') },
        { label: 'Play last', onPress: () => enqueue([track], 'end') },
        { label: 'Play alone', onPress: () => playTrackFromNewQueue([track], 0, fromMixParams) },
      ])
    )}
  {display}
  fullHeight
>
  <slot {item} {focused} />
</SimpleNavigableItem>
