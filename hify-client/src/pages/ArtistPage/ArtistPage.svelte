<script lang="ts">
  import { AlbumCardFragment, AsyncArtistPage } from '../../graphql/generated'

  import Grid from '../../organisms/Grid/Grid.svelte'
  import AlbumCard from '../../molecules/AlbumCard/AlbumCard.svelte'

  export let artistId: string

  const ALBUMS_PER_PAGE = 24

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return
    }

    const artist = await AsyncArtistPage({
      variables: {
        artistId,
        pagination: {
          after: currentPageInfo?.endCursor,
          first: ALBUMS_PER_PAGE,
        },
      },
    }).then((res) => res.data.artist)

    if (!artist) {
      throw new Error("Failed to fetch artist's data")
    }

    currentPageInfo = artist.albumParticipations.pageInfo
    albums = [...albums, ...artist.albumParticipations.nodes]
    artistName = artist.name
  }

  let artistName: string | null = null
  let albums: AlbumCardFragment[] = []
</script>

{#await feedMore()}
  <h2>Loading...</h2>
{:then _}
  <h2>Artist: {artistName}</h2>

  <h3>Albums ({albums.length})</h3>

  <Grid columns={5} lazyLoader={feedMore}>
    {#each albums as album}
      <AlbumCard {album} />
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
