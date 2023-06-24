<script lang="ts">
    import { AudioTrackFragment, SetTrackRating } from "../../graphql/generated";
    import SimpleNavigableItem from "../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte";

    export let track: AudioTrackFragment

    let initialRating = track.appOnlyRating ?? track.metadata.tags.rating

    let current = initialRating
    let updating = false
    let failed = false

    async function update() {
        const updatingWith = current

        updating = true

        const done = await SetTrackRating({ variables: { trackId: track.id, rating: updatingWith } })

        updating = false

        failed = !!done.errors
        current = updatingWith
        initialRating = updatingWith
    }

    function inc(relative: number) {
        current += relative
        failed = false
    }
</script>

<SimpleNavigableItem
    onLeft={current >= 2 ? () => inc(-2) : undefined}
    onRight={current <= 8 ? () => inc(+2) : undefined}
    onPress={() => { update() }}
>
    <div class:changed={current !== initialRating} class:updating class:failed={failed}>
        {#each [2, 4, 6, 8, 10] as value}
            {#if current >= value}
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
