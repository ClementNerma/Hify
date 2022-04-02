<script lang="ts">
  import { ApolloQueryResult } from '@apollo/client/core'
  import { useNavigate } from 'svelte-navigator'

  import { API_SERVER_URL } from '../../apollo-client'
  import NavigableView from '../../atoms/NavigableView.svelte'

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

  function generateItems(res: ApolloQueryResult<AlbumsQuery>): Grid['$$prop_def']['items'] {
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

<NavigableView>
  {#await fetchAlbums()}
    <h2>Loading...</h2>
  {:then data}
    <Grid items={generateItems(data)} />
  {:catch e}
    <h2>Failed: {e.message}</h2>
  {/await}
</NavigableView>
