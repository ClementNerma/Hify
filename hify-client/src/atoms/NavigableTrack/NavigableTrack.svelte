<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { playTrackFromFetchableQueue } from '../../stores/play-queue'

  export let tracksIds: string[]
  export let trackId: string
  export let position: number | null = null

  function play() {
    playTrackFromFetchableQueue(tracksIds, tracksIds.indexOf(trackId))
  }
</script>

<SimpleNavigableItem let:item onPress={play} {position}>
  <slot {item} />
</SimpleNavigableItem>
