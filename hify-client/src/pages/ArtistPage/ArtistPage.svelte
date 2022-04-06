<script lang="ts">
  import { AsyncArtistPage, AlbumYearStrategy } from '../../graphql/generated'

  import Grid from '../../organisms/Grid/Grid.svelte'
  import AlbumCard from '../../molecules/AlbumCard/AlbumCard.svelte'

  export let artistId: string

  const ALBUMS_PER_PAGE = 24

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return
    }

    const res = await AsyncArtistPage({
      variables: {
        artistId,
        pagination: {
          after: currentPageInfo?.endCursor,
          first: ALBUMS_PER_PAGE,
        },
        strategy: AlbumYearStrategy.IdenticalOrFirstTrack,
      },
    })

    if (!res.data.artist) {
      throw new Error("Failed to fetch artist's data")
    }

    currentPageInfo = res.data.artist.albumParticipations.pageInfo
    const newAlbums = res.data.artist.albumParticipations.edges!.map((edge) => edge!.node)

    albums = [...albums, ...newAlbums]
  }

  let albums: Array<AlbumCard['$$prop_def']['album']> = []
</script>

{#await feedMore()}
  <h2>Loading...</h2>
{:then _}
  <h2>Albums ({albums.length})</h2>

  <Grid columns={5} lazyLoader={feedMore}>
    {#each albums as album}
      <AlbumCard {album} />
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
