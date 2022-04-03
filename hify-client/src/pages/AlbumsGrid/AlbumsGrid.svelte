<script lang="ts">
  import { ApolloQueryResult } from '@apollo/client/core'
  import { useNavigate } from 'svelte-navigator'
  import AlbumCard from '../../molecules/AlbumCard/AlbumCard.svelte'

  import Grid from '../../organisms/Grid/Grid.svelte'
  import { AlbumsGridQuery, AsyncAlbumsGrid } from './AlbumsGrid.generated'

  const currentCursor: string | null = null

  const fetchAlbums = () =>
    AsyncAlbumsGrid({
      variables: {
        pagination: {
          after: currentCursor,
          first: 24,
        },
      },
    })

  function generateItems(res: ApolloQueryResult<AlbumsGridQuery>): Array<AlbumCard['$$prop_def']['album']> {
    return res.data.albums.edges!.map((edge) => edge!.node)
  }
</script>

{#await fetchAlbums()}
  <h2>Loading...</h2>
{:then data}
  <Grid columns={6}>
    {#each generateItems(data) as album}
      <AlbumCard {album} />
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
