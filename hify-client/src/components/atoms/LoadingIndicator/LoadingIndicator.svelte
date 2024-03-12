<script>
import { onMount } from 'svelte'

export let delayMillis = 1000

let visible = false

onMount(() => {
	setTimeout(() => {
		visible = true
	}, delayMillis)
})
</script>

{#if visible}
  <h2>Still loading...</h2>
{/if}
