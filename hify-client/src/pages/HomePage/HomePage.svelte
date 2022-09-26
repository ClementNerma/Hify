<script lang="ts">
  import Row from '../../atoms/Row/Row.svelte'
  import { AsyncHomePage } from '../../graphql/generated'
  import AlbumsRow from '../../molecules/AlbumsRow/AlbumsRow.svelte'
  import ArtistsRow from '../../molecules/ArtistsRow/ArtistsRow.svelte'
  import TracksRow from '../../molecules/TracksRow/TracksRow.svelte'
  import IndexUpdater from './IndexUpdater.svelte'
  import MixGenerator from './MixGenerator.svelte'

  const indexInfos = AsyncHomePage({ variables: {}, fetchPolicy: 'no-cache' }).then((res) => res.data.generateFeed)
</script>

{#await indexInfos}
  <h1>Loading...</h1>
{:then { lastListenedTo, popularTracks, popularAlbums, popularArtists, randomGreatAlbums, randomGreatArtists }}
  <h2>Tracks you like to listen to:</h2>

  <TracksRow tracks={popularTracks} />

  <h2>Last songs you listened to:</h2>

  <TracksRow tracks={lastListenedTo} />

  <h2>Albums you like to listen to:</h2>

  <AlbumsRow albums={popularAlbums} />

  <h2>Artists you like to listen to:</h2>

  <ArtistsRow artists={popularArtists} />

  <h2>Random great albums:</h2>

  <AlbumsRow albums={randomGreatAlbums} />

  <h2>Random great artists:</h2>

  <ArtistsRow artists={randomGreatArtists} />

  <h2>Tools</h2>

  <Row>
    <IndexUpdater />
    <MixGenerator />
  </Row>
{:catch e}
  <h1>Failed to load homepage: {e.message}</h1>
{/await}
