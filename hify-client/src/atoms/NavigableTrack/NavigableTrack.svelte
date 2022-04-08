<script lang="ts">
  import { useNavigate } from 'svelte-navigator'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ROUTES } from '../../routes'
  import { playTrackFromFetchableQueue } from '../../stores/play-queue'

  export let tracksIds: string[]
  export let trackId: string
  export let albumId: string
  export let position: number | null = null
  export let transparent = false

  function play() {
    playTrackFromFetchableQueue(tracksIds, tracksIds.indexOf(trackId))
    navigate(ROUTES.nowPlaying)
  }

  const navigate = useNavigate()
</script>

<SimpleNavigableItem
  let:item
  onPress={play}
  onLongPress={() => navigate(ROUTES.album(albumId))}
  {position}
  {transparent}
>
  <slot {item} />
</SimpleNavigableItem>
