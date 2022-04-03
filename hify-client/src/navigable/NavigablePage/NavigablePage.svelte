<script lang="ts">
  import { handleKeyboardEvent, usePageNavigator } from '../navigation'

  const page = usePageNavigator()

  export let onKeyDown: ((e: KeyboardEvent) => void | boolean) | undefined = undefined

  function handleWindowKeyEvent(e: KeyboardEvent) {
    if (onKeyDown?.(e) !== false) {
      handleKeyboardEvent(e)
    }
  }
</script>

<svelte:window on:keydown={handleWindowKeyEvent} />

<slot />
