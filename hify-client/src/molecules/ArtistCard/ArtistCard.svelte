<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { ArtistCardFragment } from '../../graphql/generated'
  import { ROUTES } from '../../routes'
  import { bind } from '../../globals/utils'
  import InteractiveCard from '../Card/InteractiveCard.svelte'
  import { showContextMenu } from '../../navigable/ui/molecules/ContextMenu/ContextMenu'
  import { generateAndPlayMix } from '../../atoms/MixButton/MixGenerator'
  import { MIN_GREAT_RATING } from '../../constants'

  export let artist: ArtistCardFragment

  $: contextMenuOptions = [
    {
      label: 'Mix me some magic ✨',
      onPress: bind(artist.id, (id) => generateAndPlayMix({ minRating: MIN_GREAT_RATING, fromArtists: [id] })),
    },
  ]
</script>

<InteractiveCard
  title={artist.name}
  subtitle=""
  onPress={bind(artist, (artist) => navigate(ROUTES.artist(artist.id)))}
  onLongPress={() => showContextMenu(contextMenuOptions)}
  art={artist.art}
  circle={true}
/>
