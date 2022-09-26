<script lang="ts">
  import { navigate } from 'svelte-navigator'

  import { AsyncHistory } from '../../graphql/generated'
  import InteractiveCard from '../../molecules/Card/InteractiveCard.svelte'
  import { showContextMenu } from '../../molecules/ContextMenu/ContextMenu.svelte'
  import Grid from '../../organisms/Grid/Grid.svelte'
  import { getAlbumArtUri } from '../../rest-api'
  import { ROUTES } from '../../routes'
  import { playTrackFromNewQueue } from '../../stores/play-queue'
  import { bind } from '../../utils'
  import { contextMenuStore } from '../Template/TplContextMenu.svelte'

  const history = AsyncHistory({
    variables: {},
    fetchPolicy: 'no-cache',
  })
</script>

{#await history}
  <h2>Loading...</h2>
{:then history}
  <h2>History: {history.data.history.length} tracks</h2>

  <Grid columns={6}>
    {#each history.data.history as track, i}
      {@const tags = track.metadata.tags}

      <InteractiveCard
        title={tags.title}
        subtitle={tags.album.name}
        pictureUrl={getAlbumArtUri(tags.album.id)}
        onPress={bind({ history, i }, ({ history, i }) => {
          playTrackFromNewQueue(history.data.history, i)
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
