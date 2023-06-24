<script lang="ts">
    import { AudioTrackFragment, SetTrackRating } from "../../graphql/generated";
    import SimpleNavigableItem from "../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte";
    import { bind } from "../../globals/utils";

    export let track: AudioTrackFragment
    
    let initialRating: number | null

    let current: number | null
    let updating: boolean
    let failed: boolean

    $: {
        initialRating = track.appOnlyRating ?? track.metadata.tags.rating
        current = initialRating
        updating = false
        failed = false
    }

    async function update() {
        const updatingWith = current

        updating = true

        const done = await SetTrackRating({ variables: { trackId: track.id, rating: updatingWith } })

        updating = false

        failed = !!done.errors
        current = updatingWith
        initialRating = updatingWith

        // Not ideal but required because re-fetching the whole tracks list
        // would be both complex and inefficient
        track.appOnlyRating = updatingWith
    }

    function setRating(newRating: number) {
        current = newRating
        failed = false
    }
</script>

<SimpleNavigableItem
    onLeft={current === null ? undefined : current >= 2 ? bind(current, (current) => setRating(current - 2)) : undefined}
    onRight={current === null ? () => setRating(2) : current <= 8 ? bind(current, (current) => setRating(current + 2)) : undefined}
    onPress={() => { update() }}
>
    <div class:changed={current !== initialRating} class:updating class:failed={failed}>
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
