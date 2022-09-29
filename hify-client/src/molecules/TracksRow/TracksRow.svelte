<script lang="ts">
  import Row from '../../atoms/Row/Row.svelte'
  import { AudioTrackFragment } from '../../graphql/generated'
  import TrackCard from '../TrackCard/TrackCard.svelte'

  export let tracks: AudioTrackFragment[]
</script>

<Row>
  {#each tracks as track, i (track.id)}
    <TrackCard {track} {tracks} position={i} />
  {/each}
</Row>
