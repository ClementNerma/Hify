<script lang="ts">
  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'
  import Row from '../../atoms/Row/Row.svelte'
  import { AudioTrackFragment } from '../../graphql/generated'
  import { getAlbumArtUri } from '../../rest-api'
  import Card from '../Card/Card.svelte'

  export let tracks: AudioTrackFragment[]
</script>

<Row>
  {#each tracks as track, i (track.id)}
    <NavigableTrack position={i} {tracks} {track}>
      <Card
        title={track.metadata.tags.title}
        subtitle={`${track.metadata.tags.album.name} - ${track.metadata.tags.artists
          .map((artist) => artist.name)
          .join(' / ')}`}
        pictureUrl={getAlbumArtUri(track.metadata.tags.album.id)}
        enforceMaxWidth={true}
      />
    </NavigableTrack>
  {/each}
</Row>
