<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { ArtistCardFragment } from '../../graphql/generated'
  import { getArtistArtUri } from '../../globals/rest-api'
  import { ROUTES } from '../../routes'
  import { bind } from '../../globals/utils'
  import InteractiveCard from '../Card/InteractiveCard.svelte'
  import { showContextMenu } from '../ContextMenu/ContextMenu'
  import { generateAndPlayMix } from '../../atoms/MixButton/MixGenerator'

  export let artist: ArtistCardFragment

  $: contextMenuOptions = [
    { label: 'Mix me some magic ✨', onPress: bind(artist.id, (id) => generateAndPlayMix({ fromArtist: id })) },
  ]
</script>

<InteractiveCard
  title={artist.name}
  subtitle=""
  onPress={bind(artist, (artist) => navigate(ROUTES.artist(artist.id)))}
  pictureUrl={getArtistArtUri(artist.id)}
  onLongPress={() => showContextMenu(contextMenuOptions)}
  rounded={true}
/>
