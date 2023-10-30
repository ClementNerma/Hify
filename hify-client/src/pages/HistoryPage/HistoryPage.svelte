<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AsyncHistoryPage, AudioTrackFragment } from '../../graphql/generated'
  import InteractiveCard from '../../molecules/Card/InteractiveCard.svelte'
  import { showContextMenu } from '../../navigable/ui/molecules/ContextMenu/ContextMenu'
  import Grid from '../../navigable/ui/organisms/Grid/Grid.svelte'
  import { ROUTES } from '../../routes'
  import { playTrackFromNewQueue } from '../../stores/play-queue'
  import { bind } from '../../globals/utils'
  import LoadingIndicator from '../../atoms/LoadingIndicator/LoadingIndicator.svelte'

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
      fetchPolicy: 'no-cache',
    })

    currentPageInfo = res.data.history.pageInfo
    tracks = [...tracks, ...res.data.history.nodes]
  }

  let tracks: AudioTrackFragment[] = []
</script>

{#await feedMore()}
  <LoadingIndicator />
{:then _}
  <h2>History</h2>

  <Grid columns={7} lazyLoader={feedMore}>
    {#each tracks as track, i}
      {@const tags = track.metadata.tags}

      <InteractiveCard
        title={tags.title}
        art={tags.album.art}
        onPress={bind(i, (i) => {
          playTrackFromNewQueue(tracks, i, null)
          navigate(ROUTES.nowPlaying)
        })}
        onLongPress={bind(tags.album, (album) =>
          showContextMenu([{ label: 'Go to album', onPress: () => navigate(ROUTES.album(album.id)) }])
        )}
      />
      <!-- subtitle={tags.album.name} -->
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
