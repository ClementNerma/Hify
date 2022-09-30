<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { generateAndPlayMix } from '../../atoms/MixButton/MixGenerator'
  import { AsyncGenresPage } from '../../graphql/generated'
  import { showContextMenu } from '../../molecules/ContextMenu/context-menu'
  import ItemStyleLayer from '../../navigable/SimpleNavigableItem/ItemStyleLayer.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Grid from '../../organisms/Grid/Grid.svelte'
  import { ROUTES } from '../../routes'
  import { contextMenuStore } from '../Template/TplContextMenu.svelte'

  const genres = AsyncGenresPage({ variables: {} }).then((res) => res.data.genres)

  $: contextMenuOptions = (genreId: string) => [
    { label: 'Mix me some magic ✨', onPress: () => generateAndPlayMix({ fromGenre: genreId }) },
  ]
</script>

{#await genres}
  <h2>Loading...</h2>
{:then genres}
  <h2>List of all genres ({genres.length}) and number of albums:</h2>

  <Grid columns={6}>
    {#each genres as genre}
      <SimpleNavigableItem
        onPress={() => navigate(ROUTES.genre(genre.id))}
        onLongPress={() => showContextMenu(contextMenuStore, contextMenuOptions(genre.id))}
      >
        <ItemStyleLayer>
          <p>{genre.name} ({genre.albumsCount})</p>
        </ItemStyleLayer>
      </SimpleNavigableItem>
    {/each}
  </Grid>
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
