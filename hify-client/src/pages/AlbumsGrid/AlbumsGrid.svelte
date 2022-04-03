<script lang="ts">
  import { AsyncAlbumsGrid } from './AlbumsGrid.generated'

  import AlbumCard from '../../molecules/AlbumCard/AlbumCard.svelte'
  import Grid from '../../organisms/Grid/Grid.svelte'

  const ALBUMS_PER_PAGE = 24

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return
    }

    const res = await AsyncAlbumsGrid({
      variables: {
        pagination: {
          after: currentPageInfo?.endCursor,
          first: ALBUMS_PER_PAGE,
        },
      },
    })

    currentPageInfo = res.data.albums.pageInfo
    const newAlbums = res.data.albums.edges!.map((edge) => edge!.node)

    albums = [...albums, ...newAlbums]
  }

  let albums: Array<AlbumCard['$$prop_def']['album']> = []
</script>

{#await feedMore()}
  <h2>Loading...</h2>
{:then _}
  <Grid columns={6} lazy={feedMore}>
    {#each albums as album}
      <AlbumCard {album} />
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
