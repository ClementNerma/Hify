<script lang="ts">
  import Row from '@navigable/ui/molecules/Row/Row.svelte'
  import { AudioTrackFragment } from '@graphql/generated'
  import TrackCard from '@molecules/TrackCard/TrackCard.svelte'

  export let tracks: AudioTrackFragment[]

  const INITIAL_ITEMS = 10

  let displaying = INITIAL_ITEMS

  function reached(index: number) {
    if (index >= displaying - 2) {
      displaying += INITIAL_ITEMS
    }
  }
</script>

<Row>
  {#each tracks.slice(0, displaying) as track, i (track.id)}
    <!-- This <div> allows the inner item to reach full height -->
    <div>
      <TrackCard {track} {tracks} onFocus={() => reached(i)} />
    </div>
  {/each}
</Row>
