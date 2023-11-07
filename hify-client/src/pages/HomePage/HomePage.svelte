<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import Button from '@atoms/Button/Button.svelte'
  import MixButton from '@atoms/MixButton/MixButton.svelte'
  import Row from '@navigable/ui/molecules/Row/Row.svelte'
  import { AsyncHomePage, MixOrdering } from '@graphql/generated'
  import TracksRow from '@molecules/TracksRow/TracksRow.svelte'
  import { ROUTES } from '@root/routes'
  import IndexUpdater from './IndexUpdater.svelte'
  import LoadingIndicator from '@atoms/LoadingIndicator/LoadingIndicator.svelte'
  import StatsBox from '@molecules/StatsBox/StatsBox.svelte'
  import Centered from '@atoms/Centered/Centered.svelte'
  import AlbumsRow from '@molecules/AlbumsRow/AlbumsRow.svelte'
  import { MIN_GREAT_RATING } from '@root/constants'

  async function fetchFeed() {
    const res = await AsyncHomePage({
      variables: {
        randomItemsParams: {
          minRating: 8,
          maxItems: 25,
        },
      },

      fetchPolicy: 'no-cache',
    })

    return res.data.generateFeed
  }

  function onUpdated() {
    feed = fetchFeed()
  }

  let feed = fetchFeed()
  let statsBox = false
</script>

{#await feed}
  <LoadingIndicator />
{:then { lastListenedTo, periodicallyPopularTracks, mostRecentAlbums }}
  <Centered>
    <h2>Welcome!</h2>

    <MixButton
      mixParams={{
        source: { allTracks: null },
        ordering: MixOrdering.Random,
        minRating: MIN_GREAT_RATING,
      }}
    />
  </Centered>

  <Centered>
    <h3>Tracks you currently like to listen to:</h3>
  </Centered>

  <TracksRow tracks={periodicallyPopularTracks} />

  <Centered>
    <h3>Last songs you listened to:</h3>
  </Centered>

  <TracksRow tracks={lastListenedTo} />

  <Centered>
    <h3>Last albums to collection:</h3>
  </Centered>

  <AlbumsRow albums={mostRecentAlbums} />

  <!-- <h3>Albums you like to listen to:</h3>

  <AlbumsRow albums={popularAlbums} />

  <h3>Artists you like to listen to:</h3>

  <ArtistsRow artists={popularArtists} /> -->

  <!-- <h3>Random great albums:</h3>

  <AlbumsRow albums={randomGreatAlbums} />

  <h3>Random great artists:</h3>

  <ArtistsRow artists={randomGreatArtists} /> -->

  <Centered>
    <h3>Tools</h3>
  </Centered>

  <Row>
    <IndexUpdater {onUpdated} />
    <Button onPress={() => navigate(ROUTES.devTools)} fullHeight>👷 Devtools</Button>
    <Button
      onPress={() => {
        statsBox = !statsBox
      }}>Show me some stats</Button
    >
  </Row>

  {#if statsBox}
    <Row>
      <StatsBox />
    </Row>
  {/if}
{:catch e}
  <h1>Failed to load homepage: {e.message}</h1>
{/await}
