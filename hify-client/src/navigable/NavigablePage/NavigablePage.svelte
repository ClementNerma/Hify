<script lang="ts">
  import { handleKeyboardEvent, usePageNavigator } from '../navigation'

  const page = usePageNavigator()
</script>

<svelte:window on:keydown={handleKeyboardEvent} />

<slot />
