<script lang="ts">
  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'
  import { AudioTrackFragment } from '../../graphql/generated'
  import Card from '../Card/Card.svelte'

  export let track: AudioTrackFragment
  export let tracks: AudioTrackFragment[]
</script>

<NavigableTrack {track} {tracks}>
  <Card
    title={track.metadata.tags.title}
    art={track.metadata.tags.album.art}
  />
  <!-- subtitle={`${track.metadata.tags.album.name} - ${track.metadata.tags.artists
      .map((artist) => artist.name)
      .join(' / ')}`} -->
</NavigableTrack>
