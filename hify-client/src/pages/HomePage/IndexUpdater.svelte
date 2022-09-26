<script lang="ts">
  import { IndexUpdater } from '../../graphql/generated'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'

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

<SimpleNavigableItem onPress={() => updateIndex()} disabled={isUpdating} transparent>
  <p class={isUpdating ? 'updating' : ''}>Update the index (this might take a while)</p>

  {#if isUpdating}
    ⌛
  {/if}

  {#if updateResult?.type === 'ok'}
    ✅
  {:else if updateResult?.type === 'err'}
    ❌ {updateResult.err.message}
  {/if}
</SimpleNavigableItem>

<style>
  p {
    display: inline-block;
    padding: 5px;
    margin: 0;
    border: 1px solid white;
  }

  .updating {
    opacity: 0.5;
  }
</style>
