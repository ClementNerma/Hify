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

    const res = await AsyncArtistPage({
      variables: {
        artistId,
        pagination: {
          after: currentPageInfo?.endCursor,
          first: ALBUMS_PER_PAGE,
        },
      },
    }).then((res) => res.data.artist?.albumParticipations)

    if (!res) {
      throw new Error("Failed to fetch artist's data")
    }

    currentPageInfo = res.pageInfo
    albums = [...albums, ...res.nodes]
  }

  let albums: AlbumCardFragment[] = []
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
