import { navigate } from 'svelte-navigator'
import { LARGE_MIX_TRACKS_QTY } from '../../constants'
import { AsyncMixGenerator, MixParams } from '../../graphql/generated'
import { ROUTES } from '../../routes'
import { playNewQueueFromBeginning } from '../../stores/play-queue'

export async function generateAndPlayMix(input: MixParams) {
	const mix = await AsyncMixGenerator({
		variables: {
			input,
			maxTracks: LARGE_MIX_TRACKS_QTY,
		},
		fetchPolicy: 'no-cache',
	})

	playNewQueueFromBeginning(mix.data.generateMix, input)
	navigate(ROUTES.nowPlaying)
}
