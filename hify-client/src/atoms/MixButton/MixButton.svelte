<script lang="ts">
  import Button from '../../atoms/Button/Button.svelte'
  import { MixParams } from '../../graphql/generated'
  import { generateAndPlayMix } from './MixGenerator'

  export let mixParams: MixParams
</script>

<Button onPress={() => generateAndPlayMix(mixParams)}>Mix me some magic ✨</Button>
