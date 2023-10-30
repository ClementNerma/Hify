import { ApolloQueryResult } from '@apollo/client'
import { navigate } from 'svelte-navigator'
import { LARGE_MIX_TRACKS_QTY } from '../../constants'
import { AsyncMixGenerator, MixGeneratorQuery, MixParams } from '../../graphql/generated'
import { ROUTES } from '../../routes'
import { playNewQueueFromBeginning } from '../../stores/play-queue'

export async function generateMix(input: MixParams, maxTracks: number): Promise<ApolloQueryResult<MixGeneratorQuery>> {
	return AsyncMixGenerator({
		variables: {
			input,
			maxTracks,
		},
		fetchPolicy: 'no-cache',
	})
}

export async function generateAndPlayMix(input: MixParams) {
	const mix = await generateMix(input, LARGE_MIX_TRACKS_QTY)

	playNewQueueFromBeginning(mix.data.generateMix, input)
	navigate(ROUTES.nowPlaying)
}
