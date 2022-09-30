<script lang="ts">
  import Button from '../../navigable/ui/atoms/Button/Button.svelte'
  import { IndexUpdater } from '../../graphql/generated'

  function updateIndex() {
    isUpdating = true
    updateResult = null

    IndexUpdater({ variables: {} })
      .then(() => {
        updateResult = { type: 'ok' }
      })
      .catch((err) => {
        updateResult = { type: 'err', err }
      })
      .finally(() => {
        isUpdating = false
      })
  }

  let isUpdating = false
  let updateResult: { type: 'ok' } | { type: 'err'; err: Error } | null = null
</script>

<Button onPress={() => updateIndex()} disabled={isUpdating} fullHeight
  >Update the index (this might take a while)</Button
>

<span>
  {#if isUpdating}
    ⌛
  {/if}

  {#if updateResult?.type === 'ok'}
    ✅
  {:else if updateResult?.type === 'err'}
    ❌ {updateResult.err.message}
  {/if}
</span>
