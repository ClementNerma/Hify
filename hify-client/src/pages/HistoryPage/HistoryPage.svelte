<script lang="ts">
  import { CONFIG } from '../../config'

  import { AsyncHistory } from '../../graphql/generated'
  import InteractiveCard from '../../molecules/Card/InteractiveCard.svelte'
  import Grid from '../../organisms/Grid/Grid.svelte'
  import { getAlbumArtUri } from '../../rest-api'
  import { playTrackFromNewQueue } from '../../stores/play-queue'
  import { bind } from '../../utils'

  const history = AsyncHistory({
    variables: {
      albumYearStrategy: CONFIG.albumYearStrategy,
    },
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
        onPress={bind({ history, i }, ({ history, i }) => playTrackFromNewQueue(history.data.history, i))}
        pictureUrl={getAlbumArtUri(tags.album.id)}
      />
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
