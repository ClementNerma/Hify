<script lang="ts">
  import Row from '../../navigable/ui/molecules/Row/Row.svelte'
  import { AudioTrackFragment } from '../../graphql/generated'
  import TrackCard from '../TrackCard/TrackCard.svelte'

  export let tracks: AudioTrackFragment[]
</script>

<Row>
  {#each tracks as track, i (track.id)}
    <!-- This <div> allows the inner item to reach full height -->
    <div>
      <TrackCard {track} {tracks} position={i} />
    </div>
  {/each}
</Row>
