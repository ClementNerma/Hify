<script lang="ts">
  import {
    AudioTrackFragment,
    RemoveTrackRating,
    SetTrackRating,
  } from '../../graphql/generated'
  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'

  export let track: AudioTrackFragment

  let initialRating: number

  let current: number
  let updating: boolean
  let failed: boolean

  $: {
    initialRating = track.computedRating ?? 0
    current = initialRating
    updating = false
    failed = false
  }

  async function update() {
    const updatingWith = current

    updating = true

    const done =
      updatingWith > 0
        ? await SetTrackRating({
            variables: { trackId: track.id, rating: updatingWith },
          })
        : await RemoveTrackRating({
            variables: { trackId: track.id },
          })

    updating = false

    failed = !!done.errors
    current = updatingWith
    initialRating = updatingWith

    // Not ideal but required because re-fetching the whole tracks list
    // would be both complex and inefficient
    track.computedRating = updatingWith
  }

  function setRatingRelative(rel: number) {
    current += rel
    failed = false
  }

  function reset() {
    current = initialRating
  }
</script>

<SimpleNavigableItem
  onLeft={current >= 2 ? () => setRatingRelative(-2) : undefined}
  onRight={current <= 8 ? () => setRatingRelative(+2) : undefined}
  onPress={() => void update()}
  onUnfocus={reset}
>
  <div class:changed={current !== initialRating} class:updating class:failed>
    {#each [2, 4, 6, 8, 10] as value}
      {#if current !== null && current >= value}
        &starf;
      {:else}
        &star;
      {/if}
    {/each}
  </div>
</SimpleNavigableItem>

<style>
  .changed {
    color: purple;
  }

  .updating {
    color: gray;
    opacity: 0.5;
  }

  .failed {
    color: red;
  }
</style>
