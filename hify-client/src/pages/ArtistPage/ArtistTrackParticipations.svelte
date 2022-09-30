<script lang="ts">
  import { AsyncArtistTrackParticipations, AudioTrackFragment } from '../../graphql/generated'

  import Grid from '../../organisms/Grid/Grid.svelte'
  import Button from '../../atoms/Button/Button.svelte'
  import TrackCard from '../../molecules/TrackCard/TrackCard.svelte'

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
  {#if tracks.length !== 0}
    <h3>Tracks from other artists' albums ({tracks.length})</h3>

    <Grid columns={TRACKS_PER_LINE}>
      {#each tracks as track}
        <TrackCard {track} {tracks} />
      {/each}
    </Grid>

    {#if lastPageInfo.hasNextPage}
      <Button onPress={() => feedMore()}>Load more</Button>
    {/if}
  {/if}
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
