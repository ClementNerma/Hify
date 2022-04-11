<script lang="ts">
  import { useNavigate } from 'svelte-navigator'
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

  function play() {
    playTrackFromNewQueue(tracks, tracks.indexOf(track))
    navigate(ROUTES.nowPlaying)
  }

  const navigate = useNavigate()
</script>

<SimpleNavigableItem
  let:item
  onPress={play}
  onLongPress={() =>
    showContextMenu(contextMenuStore, [
      { label: 'Go to album', onPress: () => navigate(ROUTES.album(track.metadata.tags.album.id)) },
      { label: 'Play next', onPress: () => queueAsNext(track) },
    ])}
  {position}
  {transparent}
>
  <slot {item} />
</SimpleNavigableItem>
