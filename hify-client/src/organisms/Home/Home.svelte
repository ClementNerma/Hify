<script lang="ts">
  import { AsyncIndex } from './Index.generated'

  const index = AsyncIndex({
    variables: {
      fingerprint: 'NOPE',
    },
  }).then((index) => index.data.index)
</script>

{#await index}
  <h1>Loading...</h1>
{:then index}
  {#if index}
    <h1>Number of tracks: {index?.tracks.length}</h1>
  {:else}
    <h1>Index was not generated yet.</h1>
  {/if}
{:catch e}
  <h1>Failed: {e.message}</h1>
{/await}
