<script lang="ts">
  import Row from '@navigable/ui/molecules/Row/Row.svelte'
  import { AudioTrackFragment } from '@graphql/generated'
  import TrackCard from '@molecules/TrackCard/TrackCard.svelte'

  export let tracks: AudioTrackFragment[]
</script>

<Row>
  {#each tracks as track (track.id)}
    <!-- This <div> allows the inner item to reach full height -->
    <div>
      <TrackCard {track} {tracks} />
    </div>
  {/each}
</Row>
