<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AsyncGenresPage } from '../../graphql/generated'
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { ROUTES } from '../../routes'

  const genres = AsyncGenresPage({ variables: {} }).then((res) => res.data.genres)
</script>

{#await genres}
  <h2>Loading...</h2>
{:then genres}
  <NavigableList>
    {#each genres as genre}
      <SimpleNavigableItem onPress={() => navigate(ROUTES.genre(genre.id))} transparent={true}>
        <p class="genre">{genre.name}</p>
      </SimpleNavigableItem>
    {/each}
  </NavigableList>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}

<style>
  .genre {
    padding: 10px;
  }
</style>
