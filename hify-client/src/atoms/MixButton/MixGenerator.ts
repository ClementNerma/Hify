import { navigate } from 'svelte-navigator'
import { AsyncMixGenerator, MixParams } from '../../graphql/generated'
import { ROUTES } from '../../routes'
import { playNewQueueFromBeginning } from '../../stores/play-queue'

export async function generateAndPlayMix(input: MixParams) {
  const mix = await AsyncMixGenerator({
    variables: {
      input,
    },
    fetchPolicy: 'no-cache',
  })

  if (mix.data.generateMix) {
    playNewQueueFromBeginning(mix.data.generateMix)
    navigate(ROUTES.nowPlaying)
  }
}
