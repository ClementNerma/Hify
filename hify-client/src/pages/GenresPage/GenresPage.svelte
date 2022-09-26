<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AsyncGenresPage } from '../../graphql/generated'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Grid from '../../organisms/Grid/Grid.svelte'
  import { ROUTES } from '../../routes'

  const genres = AsyncGenresPage({ variables: {} }).then((res) => res.data.genres)
</script>

{#await genres}
  <h2>Loading...</h2>
{:then genres}
  <h2>List of all genres ({genres.length}) and number of albums:</h2>

  <Grid columns={6}>
    {#each genres as genre}
      <SimpleNavigableItem onPress={() => navigate(ROUTES.genre(genre.id))} transparent={true}>
        <p class="genre">{genre.name} ({genre.albumsCount})</p>
      </SimpleNavigableItem>
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}

<style>
  .genre {
    padding: 10px;
  }
</style>
