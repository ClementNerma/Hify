<script lang="ts">
  import { AsyncArtistTrackParticipations, AudioTrackFragment } from '../../graphql/generated'

  import Grid from '../../organisms/Grid/Grid.svelte'
  import Button from '../../atoms/Button/Button.svelte'
  import NavigableTrack from '../../atoms/NavigableTrack/NavigableTrack.svelte'
  import Card from '../../molecules/Card/Card.svelte'
  import { getAlbumArtUri } from '../../rest-api'

  export let artistId: string

  const TRACKS_PER_LINE = 5
  const LINES_PER_REQUEST = 5

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return currentPageInfo
    }

    const res = await AsyncArtistTrackParticipations({
      variables: {
        artistId,
        pagination: {
          after: currentPageInfo?.endCursor,
          first: TRACKS_PER_LINE * LINES_PER_REQUEST,
        },
      },
    }).then((res) => res.data.artist?.trackParticipations)

    if (!res) {
      throw new Error("Failed to fetch artist's data")
    }

    currentPageInfo = res.pageInfo
    tracks = [...tracks, ...res.nodes]

    return currentPageInfo
  }

  let tracks: AudioTrackFragment[] = []
</script>

{#await feedMore()}
  <h3>Loading...</h3>
{:then lastPageInfo}
  <h3>Tracks from other artists' albums ({tracks.length})</h3>

  <Grid columns={TRACKS_PER_LINE}>
    {#each tracks as track, i}
      <NavigableTrack {track} {tracks} position={i}>
        <Card
          title={track.metadata.tags.title}
          subtitle={`${track.metadata.tags.album.name} - ${track.metadata.tags.artists
            .map((artist) => artist.name)
            .join(' / ')}`}
          pictureUrl={getAlbumArtUri(track.metadata.tags.album.id)}
        />
      </NavigableTrack>
    {/each}
  </Grid>

  {#if lastPageInfo.hasNextPage}
    <Button onPress={() => feedMore()}>Load more</Button>
  {/if}
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
