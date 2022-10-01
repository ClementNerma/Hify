<script lang="ts">
  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'
  import { AudioTrackFragment } from '../../graphql/generated'
  import { getAlbumArtUri } from '../../globals/rest-api'
  import Card from '../Card/Card.svelte'

  export let track: AudioTrackFragment
  export let tracks: AudioTrackFragment[]
</script>

<NavigableTrack {track} {tracks}>
  <Card
    title={track.metadata.tags.title}
    subtitle={`${track.metadata.tags.album.name} - ${track.metadata.tags.artists
      .map((artist) => artist.name)
      .join(' / ')}`}
    pictureUrl={getAlbumArtUri(track.metadata.tags.album.id)}
  />
</NavigableTrack>
