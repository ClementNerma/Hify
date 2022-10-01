<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AudioTrackFragment } from '../../graphql/generated'
  import { showContextMenu } from '../../navigable/ui/molecules/ContextMenu/ContextMenu'

  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ItemDisplay } from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ROUTES } from '../../routes'
  import { playTrackFromNewQueue, queueAsNext } from '../../stores/play-queue'

  export let tracks: AudioTrackFragment[]
  export let track: AudioTrackFragment
  export let goToAlbumOption = true
  export let display: ItemDisplay = null

  function play() {
    playTrackFromNewQueue(tracks, tracks.indexOf(track))
    navigate(ROUTES.nowPlaying)
  }
</script>

<SimpleNavigableItem
  let:item
  onPress={play}
  onLongPress={() =>
    showContextMenu(
      (goToAlbumOption
        ? [{ label: 'Go to album', onPress: () => navigate(ROUTES.album(track.metadata.tags.album.id)) }]
        : []
      ).concat([
        { label: 'Play next', onPress: () => queueAsNext([track]) },
        { label: 'Play alone', onPress: () => playTrackFromNewQueue([track], 0) },
      ]),
    )}
  {display}
  fullHeight
>
  <slot {item} />
</SimpleNavigableItem>
