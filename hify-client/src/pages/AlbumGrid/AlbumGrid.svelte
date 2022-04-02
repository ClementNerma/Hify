<script lang="ts">
  import { ApolloQueryResult } from '@apollo/client/core'
  import { useNavigate } from 'svelte-navigator'

  import { API_SERVER_URL } from '../../apollo-client'
  import Card from '../../molecules/Card/Card.svelte'

  import Grid from '../../organisms/Grid/Grid.svelte'
  import { AlbumsQuery, AsyncAlbums } from './Albums.generated'

  const nav = useNavigate()

  const currentCursor: string | null = null

  const fetchAlbums = () =>
    AsyncAlbums({
      variables: {
        pagination: {
          after: currentCursor,
          first: 24,
        },
      },
    })

  function generateItems(res: ApolloQueryResult<AlbumsQuery>): Card['$$prop_def'][] {
    return res.data.albums.edges!.map((edge) => {
      const node = edge!.node

      return {
        title: node.name,
        subtitle: node.albumArtists.map((artist) => artist.name).join(', '),
        onPress: () => nav(`/album/${node.id}`),
        onSubtitleClick: () => alert('TODO: go to artists page'),
        pictureUrl: `${API_SERVER_URL}/art/${node.id}`,
        pictureAlt: 'Album Art',
      }
    })
  }
</script>

{#await fetchAlbums()}
  <h2>Loading...</h2>
{:then data}
  <Grid columns={6}>
    {#each generateItems(data) as item}
      <Card {...item} />
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
