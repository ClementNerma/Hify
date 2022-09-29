<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import Button from '../../atoms/Button/Button.svelte'
  import Row from '../../atoms/Row/Row.svelte'
  import Spacer from '../../atoms/Spacer/Spacer.svelte'
  import { AsyncHomePage } from '../../graphql/generated'
  import AlbumsRow from '../../molecules/AlbumsRow/AlbumsRow.svelte'
  import ArtistsRow from '../../molecules/ArtistsRow/ArtistsRow.svelte'
  import TracksRow from '../../molecules/TracksRow/TracksRow.svelte'
  import { ROUTES } from '../../routes'
  import IndexUpdater from './IndexUpdater.svelte'
  import MixGenerator from './MixGenerator.svelte'

  const feed = AsyncHomePage({
    variables: {
      input: {},
    },

    fetchPolicy: 'no-cache',
  }).then((res) => res.data.generateFeed)
</script>

{#await feed}
  <h1>Loading...</h1>
{:then { lastListenedTo, popularTracks, popularAlbums, popularArtists, randomGreatAlbums, randomGreatArtists }}
  <h2>Welcome!</h2>

  <MixGenerator />

  <Row>
    <IndexUpdater />
    <Button onPress={() => navigate(ROUTES.devTools)} fullHeight>👷 Go the developper's tools page</Button>
  </Row>

  <h3>Tracks you like to listen to:</h3>

  <TracksRow tracks={popularTracks} />

  <h3>Last songs you listened to:</h3>

  <TracksRow tracks={lastListenedTo} />

  <h3>Albums you like to listen to:</h3>

  <AlbumsRow albums={popularAlbums} />

  <h3>Artists you like to listen to:</h3>

  <ArtistsRow artists={popularArtists} />

  <h3>Random great albums:</h3>

  <AlbumsRow albums={randomGreatAlbums} />

  <h3>Random great artists:</h3>

  <ArtistsRow artists={randomGreatArtists} />

  <h3>Tools</h3>

  <Spacer />
{:catch e}
  <h1>Failed to load homepage: {e.message}</h1>
{/await}
