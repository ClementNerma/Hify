<script lang="ts">
  import { navigate } from "svelte-navigator";
  import Button from "../../navigable/ui/atoms/Button/Button.svelte";
  import MixButton from "../../atoms/MixButton/MixButton.svelte";
  import Row from "../../navigable/ui/molecules/Row/Row.svelte";
  import { AsyncHomePage } from "../../graphql/generated";
  import TracksRow from "../../molecules/TracksRow/TracksRow.svelte";
  import { ROUTES } from "../../routes";
  import IndexUpdater from "./IndexUpdater.svelte";
  import LoadingIndicator from "../../atoms/LoadingIndicator/LoadingIndicator.svelte";
  import StatsBox from "../../molecules/StatsBox/StatsBox.svelte";
    import Centered from "../../atoms/Centered/Centered.svelte";

  const feed = AsyncHomePage({
    variables: {
      input: {},
    },

    fetchPolicy: "no-cache",
  }).then((res) => res.data.generateFeed);

  let statsBox = false;
</script>

<Centered>
  {#await feed}
    <LoadingIndicator />
  {:then { lastListenedTo, periodicallyPopularTracks, randomGreatAlbums, randomGreatArtists }}
    <h2>Welcome!</h2>

    <MixButton mixParams={{}} />

    <h3>Tracks you currently like to listen to:</h3>

    <TracksRow tracks={periodicallyPopularTracks} />

    <h3>Last songs you listened to:</h3>

    <TracksRow tracks={lastListenedTo} />

    <!-- <h3>Albums you like to listen to:</h3>

    <AlbumsRow albums={popularAlbums} />

    <h3>Artists you like to listen to:</h3>

    <ArtistsRow artists={popularArtists} /> -->

    <!-- <h3>Random great albums:</h3>

    <AlbumsRow albums={randomGreatAlbums} />

    <h3>Random great artists:</h3>

    <ArtistsRow artists={randomGreatArtists} /> -->

    <h3>Tools</h3>

    <Row>
      <IndexUpdater />
      <Button onPress={() => navigate(ROUTES.devTools)} fullHeight
        >👷 Devtools</Button
      >
      <Button
        onPress={() => {
          statsBox = true;
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
</Centered>
