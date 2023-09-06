<script lang="ts">
  import LoadingIndicator from '../../atoms/LoadingIndicator/LoadingIndicator.svelte'
  import MixButton from '../../atoms/MixButton/MixButton.svelte'
  import { AsyncArtistPage } from '../../graphql/generated'

  import ArtistAlbums from './ArtistAlbums.svelte'
  import ArtistTrackParticipations from './ArtistTrackParticipations.svelte'

  export let artistId: string

  const artist = AsyncArtistPage({
    variables: { artistId },
  }).then((res) => res.data.artist)
</script>

{#await artist}
  <LoadingIndicator />
{:then artist}
  {#if !artist}
    <h2>Artist was not found!</h2>
  {:else}
    <h2>Artist: {artist.name}</h2>

    <MixButton mixParams={{ minRating: 8, maxTracks: 100, fromArtist: artistId }} />

    <ArtistAlbums {artistId} />

    <ArtistTrackParticipations {artistId} />
  {/if}
{:catch e}
  <h2>Failed: {e.message}</h2>
{/await}
