<script lang="ts">
  import { AlbumCardFragment, AsyncGenrePage } from '../../graphql/generated'

  import Grid from '../../navigable/ui/organisms/Grid/Grid.svelte'
  import AlbumCard from '../../molecules/AlbumCard/AlbumCard.svelte'
  import MixButton from '../../atoms/MixButton/MixButton.svelte'
  import LoadingIndicator from '../../atoms/LoadingIndicator/LoadingIndicator.svelte'
  import { LARGE_MIX_TRACKS_QTY, MIN_GREAT_RATING } from '../../constants'

  const ALBUMS_PER_LINE = 6
  const LINES_PER_PAGE = 5

  export let genreId: string

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return
    }

    const res = await AsyncGenrePage({
      variables: {
        genreId,
        pagination: {
          after: currentPageInfo?.endCursor,
          first: ALBUMS_PER_LINE * LINES_PER_PAGE,
        },
      },
    })

    if (!res.data.genre) {
      genreNotFound = true
      return
    }

    genreName = res.data.genre.name
    currentPageInfo = res.data.genre.albums.pageInfo
    const newAlbums = res.data.genre.albums.nodes

    albums = [...albums, ...newAlbums]
  }

  let albums: AlbumCardFragment[] = []
  let genreName: string | null = null
  let genreNotFound: boolean | null = null
</script>

{#await feedMore()}
  <LoadingIndicator />
{:then _}
  {#if genreNotFound}
    <h2>Genre was not found!</h2>
  {:else}
    <h2>Genre: {genreName}</h2>
    <MixButton mixParams={{ minRating: MIN_GREAT_RATING, fromGenres: [genreId] }} />

    <h3>List of albums</h3>

    <Grid columns={ALBUMS_PER_LINE} lazyLoader={feedMore}>
      {#each albums as album}
        <AlbumCard {album} />
      {/each}
    </Grid>
  {/if}
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
