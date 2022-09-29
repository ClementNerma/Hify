<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AsyncHistoryPage, AudioTrackFragment } from '../../graphql/generated'
  import InteractiveCard from '../../molecules/Card/InteractiveCard.svelte'
  import { showContextMenu } from '../../molecules/ContextMenu/ContextMenu.svelte'
  import Grid from '../../organisms/Grid/Grid.svelte'
  import { getAlbumArtUri } from '../../rest-api'
  import { ROUTES } from '../../routes'
  import { playTrackFromNewQueue } from '../../stores/play-queue'
  import { bind } from '../../utils'
  import { contextMenuStore } from '../Template/TplContextMenu.svelte'

  const TRACKS_PER_LINE = 7
  const LINES_PER_PAGE = 5

  let currentPageInfo: { endCursor?: string | null; hasNextPage: boolean } | null = null

  const feedMore = async () => {
    if (currentPageInfo?.hasNextPage === false) {
      return
    }

    const res = await AsyncHistoryPage({
      variables: {
        pagination: {
          after: currentPageInfo?.endCursor,
          first: TRACKS_PER_LINE * LINES_PER_PAGE,
        },
      },
    })

    currentPageInfo = res.data.history.pageInfo
    tracks = [...tracks, ...res.data.history.nodes]
  }

  let tracks: AudioTrackFragment[] = []
</script>

{#await feedMore()}
  <h2>Loading...</h2>
{:then _}
  <h2>History</h2>

  <Grid columns={7}>
    {#each tracks as track, i}
      {@const tags = track.metadata.tags}

      <InteractiveCard
        title={tags.title}
        subtitle={tags.album.name}
        pictureUrl={getAlbumArtUri(tags.album.id)}
        onPress={bind(i, (i) => {
          playTrackFromNewQueue(tracks, i)
          navigate(ROUTES.nowPlaying)
        })}
        onLongPress={bind(tags.album, (album) =>
          showContextMenu(contextMenuStore, [
            { label: 'Go to album', onPress: () => navigate(ROUTES.album(album.id)) },
          ]),
        )}
      />
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
