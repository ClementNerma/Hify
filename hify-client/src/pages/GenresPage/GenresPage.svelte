<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { generateAndPlayMix } from '../../atoms/MixButton/MixGenerator'
  import { AsyncGenresPage } from '../../graphql/generated'
  import { showContextMenu } from '../../navigable/ui/molecules/ContextMenu/ContextMenu'
  import ItemStyleLayer from '../../navigable/headless/SimpleNavigableItem/ItemStyleLayer.svelte'
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import Grid from '../../navigable/ui/organisms/Grid/Grid.svelte'
  import { ROUTES } from '../../routes'
  import LoadingIndicator from '../../atoms/LoadingIndicator/LoadingIndicator.svelte'
  import { MIN_GREAT_RATING } from '../../constants'

  const genres = AsyncGenresPage({ variables: {} }).then((res) => res.data.genres)
</script>

{#await genres}
  <LoadingIndicator />
{:then genres}
  <h2>List of all genres ({genres.length}) and number of albums:</h2>

  <Grid columns={6}>
    {#each genres as genre}
      <SimpleNavigableItem
        onPress={() => navigate(ROUTES.genre(genre.id))}
        onLongPress={() =>
          showContextMenu([
            {
              label: 'Mix me some magic ✨',
              onPress: () =>
                generateAndPlayMix({
                  minRating: MIN_GREAT_RATING,
                  fromGenres: [genre.id],
                }),
            },
          ])}
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
