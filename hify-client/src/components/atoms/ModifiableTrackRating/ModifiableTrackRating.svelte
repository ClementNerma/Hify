<script lang="ts">
import { RemoveTrackRating, SetTrackRating, type AudioTrackFragment } from '@graphql/generated'
import SimpleNavigableItem from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
import { beforeUpdate } from 'svelte'

export let track: AudioTrackFragment

let prevTrackId: string | null = null
let initialRating = 0
let current = 0
let updating = false
let failed = false

beforeUpdate(() => {
	if (prevTrackId !== track.id) {
		prevTrackId = track.id
		initialRating = track.computedRating ?? 0
		current = track.computedRating ?? 0
		updating = false
		failed = false
	}
})

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
