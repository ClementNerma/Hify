<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { ArtistInfos } from '../../graphql/generated'
  import { getArtistArtUri } from '../../rest-api'
  import { ROUTES } from '../../routes'
  import { bind } from '../../utils'
  import InteractiveCard from '../Card/InteractiveCard.svelte'

  export let artist: Pick<ArtistInfos, 'id' | 'name'>
</script>

<InteractiveCard
  title={artist.name}
  subtitle=""
  onPress={bind(artist, (artist) => navigate(ROUTES.artist(artist.id)))}
  pictureUrl={getArtistArtUri(artist.id)}
  rounded={true}
/>
