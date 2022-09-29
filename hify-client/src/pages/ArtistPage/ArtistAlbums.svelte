<script lang="ts">
  import { AlbumCardFragment, AsyncArtistAlbums } from '../../graphql/generated'

  import Grid from '../../organisms/Grid/Grid.svelte'
  import AlbumCard from '../../molecules/AlbumCard/AlbumCard.svelte'
  import Button from '../../atoms/Button/Button.svelte'

  export let artistId: string

  const ALBUMS_PER_LINE = 5
  const LINES_PER_REQUEST = 5

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return currentPageInfo
    }

    const res = await AsyncArtistAlbums({
      variables: {
        artistId,
        pagination: {
          after: currentPageInfo?.endCursor,
          first: ALBUMS_PER_LINE * LINES_PER_REQUEST,
        },
      },
    }).then((res) => res.data.artist?.albums)

    if (!res) {
      throw new Error("Failed to fetch artist's data")
    }

    currentPageInfo = res.pageInfo
    albums = [...albums, ...res.nodes]

    return currentPageInfo
  }

  let albums: AlbumCardFragment[] = []
</script>

{#await feedMore()}
  <h3>Loading...</h3>
{:then lastPageInfo}
  {#if albums.length === 0}
    <h3>No album</h3>
  {:else}
    <h3>Albums ({albums.length})</h3>

    <Grid columns={ALBUMS_PER_LINE}>
      {#each albums as album}
        <AlbumCard {album} />
      {/each}
    </Grid>

    {#if lastPageInfo.hasNextPage}
      <Button onPress={() => feedMore()}>Load more</Button>
    {/if}
  {/if}
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
