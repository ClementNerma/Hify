<script lang="ts">
  import InteractiveCard from '../../molecules/Card/InteractiveCard.svelte'
  import Grid from '../../navigable/ui/organisms/Grid/Grid.svelte'
  import { navigate } from 'svelte-navigator'
  import { AsyncHistoryPage, AudioTrackFragment, PlaylistPageQuery } from '../../graphql/generated'
  import { showContextMenu } from '../../navigable/ui/molecules/ContextMenu/ContextMenu'
  import { ROUTES } from '../../routes'
  import { playTrackFromNewQueue } from '../../stores/play-queue'
  import { bind } from '../../globals/utils'

  import { NavigableGridProps } from '../../navigable/headless/NavigableGrid/NavigableGrid'

  const TRACKS_PER_LINE = 7

  export let tracks: AudioTrackFragment[] = []
  export let feedMore: NavigableGridProps['lazyLoader']
</script>

<Grid columns={TRACKS_PER_LINE} lazyLoader={feedMore}>
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
