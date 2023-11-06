import { ApolloQueryResult } from '@apollo/client/core'
import { AsyncMixGenerator, MixGeneratorQuery, MixParams } from '@graphql/generated'
import { LARGE_MIX_TRACKS_QTY } from '@root/constants'
import { ROUTES } from '@root/routes'
import { playNewQueueFromBeginning } from '@stores/play-queue'
import { navigate } from 'svelte-navigator'

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
