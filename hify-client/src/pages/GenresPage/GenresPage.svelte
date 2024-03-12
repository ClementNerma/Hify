<script lang="ts">
import LoadingIndicator from '@atoms/LoadingIndicator/LoadingIndicator.svelte'
import { AsyncGenresPage, MixOrdering } from '@graphql/generated'
import ItemStyleLayer from '@navigable/headless/SimpleNavigableItem/ItemStyleLayer.svelte'
import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
import { showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
import Grid from '@navigable/ui/organisms/Grid/Grid.svelte'
import { MIN_GREAT_RATING } from '@root/constants'
import { ROUTES } from '@root/routes'
import { navigate } from 'svelte-navigator'
import { generateAndPlayMix } from '../../stores/play-queue'

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
                void generateAndPlayMix({
                  source: { allTracks: '-' },
                  ordering: MixOrdering.Random,
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
