<script lang="ts">
  import Button from '../../navigable/ui/atoms/Button/Button.svelte'
  import { MixParams } from '../../graphql/generated'
  import { generateAndPlayMix } from './MixGenerator'
  import { showContextMenu } from '../../navigable/ui/molecules/ContextMenu/ContextMenu'

  export let mixParams: MixParams
</script>

<Button
  onPress={() => generateAndPlayMix(mixParams)}
  onLongPress={() =>
    showContextMenu([
      {
        label: 'Only with the best rating <3',
        onPress: () => generateAndPlayMix({ ...mixParams, minRating: 100 }),
      },
    ])}>Mix me some magic ✨</Button
>
