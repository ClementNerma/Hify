<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import Button from '../../atoms/Button/Button.svelte'
  import { AsyncMixGenerator } from '../../graphql/generated'
  import { ROUTES } from '../../routes'
  import { playNewQueueFromBeginning } from '../../stores/play-queue'

  async function generateAndPlayMix() {
    const mix = await AsyncMixGenerator({
      variables: {
        input: {},
      },
      fetchPolicy: 'no-cache',
    })

    if (mix.data.generateMix) {
      playNewQueueFromBeginning(mix.data.generateMix)
      navigate(ROUTES.nowPlaying)
    }
  }
</script>

<Button onPress={() => generateAndPlayMix()} fullHeight>Mix some magic ✨</Button>
