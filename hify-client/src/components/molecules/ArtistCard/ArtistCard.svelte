<script lang="ts">
import { bind } from '@globals/utils'
import { type ArtistCardFragment, MixOrdering } from '@graphql/generated'
import { type ContextMenuOption, showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
import { MIN_GREAT_RATING } from '@root/constants'
import { ROUTES } from '@root/routes'
import { navigate } from 'svelte-navigator'
import { generateAndPlayMix } from '../../../stores/play-queue'
import InteractiveCard from '../Card/InteractiveCard.svelte'

export let artist: ArtistCardFragment

$: contextMenuOptions = [
	{
		label: 'Mix me some magic ✨',
		onPress: bind(
			artist.id,
			(id) =>
				void generateAndPlayMix({
					source: { allTracks: '-' },
					ordering: MixOrdering.Random,
					minRating: MIN_GREAT_RATING,
					fromArtists: [id],
				}),
		),
	},
] satisfies ContextMenuOption[]
</script>

<InteractiveCard
  title={artist.name}
  subtitle=""
  onPress={bind(artist, (artist) => navigate(ROUTES.artist(artist.id)))}
  onLongPress={() => showContextMenu(contextMenuOptions)}
  art={artist.art}
  circle={true}
/>
