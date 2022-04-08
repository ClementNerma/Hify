<script lang="ts">
  import { useNavigate } from 'svelte-navigator'
  import { AudioTrackFragment } from '../../graphql/generated'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ROUTES } from '../../routes'
  import { playTrackFromNewQueue } from '../../stores/play-queue'

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
  onLongPress={() => navigate(ROUTES.album(track.metadata.tags.album.id))}
  {position}
  {transparent}
>
  <slot {item} />
</SimpleNavigableItem>
