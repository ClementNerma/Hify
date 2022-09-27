<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AudioTrackFragment } from '../../graphql/generated'
  import { showContextMenu } from '../../molecules/ContextMenu/ContextMenu.svelte'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { contextMenuStore } from '../../pages/Template/TplContextMenu.svelte'
  import { ROUTES } from '../../routes'
  import { playTrackFromNewQueue, queueAsNext } from '../../stores/play-queue'

  export let tracks: AudioTrackFragment[]
  export let track: AudioTrackFragment
  export let position: number | null = null
  export let transparent = false
  export let goToAlbumOption = true

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
      contextMenuStore,
      (goToAlbumOption
        ? [{ label: 'Go to album', onPress: () => navigate(ROUTES.album(track.metadata.tags.album.id)) }]
        : []
      ).concat([
        { label: 'Play next', onPress: () => queueAsNext([track]) },
        { label: 'Play alone', onPress: () => playTrackFromNewQueue([track], 0) },
      ]),
    )}
  {position}
  {transparent}
>
  <slot {item} />
</SimpleNavigableItem>
