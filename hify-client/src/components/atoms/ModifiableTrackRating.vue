<script setup lang="ts">
import { gqlClient } from '@/global/urql-client'
import { graphql } from '@/graphql/generated'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import { computed, onBeforeUpdate, onUpdated, ref } from 'vue'

const props = defineProps<{ track: AudioTrackFragment }>()

const state = ref({
    prevTrackId: null as string | null,
    initialRating: 0,
    current: 0,
    updating: false,
    failed: false
})

onUpdated(() => {
    if (state.value.prevTrackId !== props.track.id) {
        state.value = {
            prevTrackId: props.track.id,
            initialRating: props.track.computedRating ?? 0,
            current: props.track.computedRating ?? 0,
            updating: false,
            failed: false
        }
    }
})

async function update() {
    const updatingWith = state.value.current

    state.value.updating = true

    const query = updatingWith > 0 ?
        gqlClient.mutation(graphql(`
            mutation SetTrackRating($trackId: String!, $rating: Rating!) {
                setTrackRating(trackId: $trackId, rating: $rating)
            }
        `), {
            trackId: props.track.id,
            rating: updatingWith
        }) :
        gqlClient.mutation(graphql(`
            mutation RemoveTrackRating($trackId: String!) {
               removeTrackRating(trackId: $trackId)
            }
        `), {
            trackId: props.track.id
        })

    const done = await query

    state.value = {
        prevTrackId: state.value.prevTrackId,
        updating: false,
        failed: !done.error,
        current: updatingWith,
        initialRating: updatingWith
    }

    // Not ideal but required because re-fetching the whole tracks list
    // would be both complex and inefficient
    props.track.computedRating = updatingWith
}

function setRatingRelative(rel: number) {
    state.value.current += rel
    state.value.failed = false
}

function reset() {
    state.value.current = state.value.initialRating
}

const onLeft = computed(() => state.value.current >= 2 ? () => setRatingRelative(-2) : undefined)
const onRight = computed(() => state.value.current <= 8 ? () => setRatingRelative(+2) : undefined)

</script>

<template>
    <SimpleNavigableItem :on-left :on-right @press="update()" @unfocus="reset">
        <div
            :class="state.updating ? ['text-gray', 'opacity-50'] : state.current !== state.initialRating ? ['text-purple-950'] : state.failed ? ['text-red'] : []">
            <span v-for="value in [2, 4, 6, 8, 10]">
                <span v-if="state.current !== null && state.current >= value">
                    &starf;
                </span>
                <span v-else>
                    &star;
                </span>
            </span>
        </div>
    </SimpleNavigableItem>
</template>
