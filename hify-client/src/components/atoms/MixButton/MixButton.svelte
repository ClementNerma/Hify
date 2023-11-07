<script lang="ts">
  import Button from '@atoms/Button/Button.svelte'
  import { MixParams } from '@graphql/generated'
  import { showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
  import { generateAndPlayMix } from '@stores/play-queue'

  export let mixParams: MixParams
</script>

<Button
  onPress={() => generateAndPlayMix(mixParams)}
  onLongPress={() =>
    showContextMenu([
      {
        label: 'Only with the best rating <3',
        onPress: () => generateAndPlayMix({ ...mixParams, minRating: 10 }),
      },
    ])}>Mix me some magic ✨</Button
>
