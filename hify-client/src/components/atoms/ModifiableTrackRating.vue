<script setup lang="ts">
import { computed, onBeforeUpdate, onMounted, ref } from 'vue'
import { gqlClient } from '@/global/urql-client'
import { noParallel } from '@/global/utils'
import { graphql } from '@/graphql/generated'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import { type InputHandlingResult, NavigationDirection } from '@/navigable'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'

const props = defineProps<{ track: AudioTrackFragment }>()

const state = ref({
	prevTrackId: null as string | null,
	initialRating: 0,
	current: 0,
	updating: false,
	failed: false,
})

function onComponentUpdate() {
	if (state.value.prevTrackId !== props.track.id) {
		state.value = {
			prevTrackId: props.track.id,
			initialRating: props.track.computedRating ?? 0,
			current: props.track.computedRating ?? 0,
			updating: false,
			failed: false,
		}
	}
}

onMounted(onComponentUpdate)
onBeforeUpdate(onComponentUpdate)

const modifying = ref(false)

async function onPress() {
	if (!modifying.value) {
		modifying.value = true
		return
	}

	update().finally(() => {
		modifying.value = false
	})
}

const update = noParallel(async () => {
	const updatingWith = state.value.current

	state.value.updating = true

	const query =
		updatingWith > 0
			? gqlClient.mutation(
					graphql(`
            mutation SetTrackRating($trackId: String!, $rating: Rating!) {
                setTrackRating(trackId: $trackId, rating: $rating)
            }
        `),
					{
						trackId: props.track.id,
						rating: updatingWith,
					},
				)
			: gqlClient.mutation(
					graphql(`
            mutation RemoveTrackRating($trackId: String!) {
               removeTrackRating(trackId: $trackId)
            }
        `),
					{
						trackId: props.track.id,
					},
				)

	const done = await query

	state.value = {
		prevTrackId: state.value.prevTrackId,
		updating: false,
		failed: !done.error,
		current: updatingWith,
		initialRating: updatingWith,
	}

	// Not ideal but required because re-fetching the whole tracks list
	// would be both complex and inefficient
	props.track.computedRating = updatingWith
})

function setRatingRelative(rel: number) {
	state.value.current = Math.max(0, Math.min(10, state.value.current + rel))
	state.value.failed = false
}

function onUnfocus() {
	state.value.current = state.value.initialRating
}

function interceptKeyPress(dir: NavigationDirection | null) {
	if (!modifying.value) {
		return false
	}

	if (dir === NavigationDirection.Left) {
		setRatingRelative(-2)
		return true
	}

	if (dir === NavigationDirection.Right) {
		setRatingRelative(+2)
		return true
	}

	if (dir === NavigationDirection.Back) {
		state.value.current = state.value.initialRating
		modifying.value = false
		return true
	}

	return false
}
</script>

<template>
    <NavigableItem :on-press :intercept-key-press :on-unfocus>
        <div :class="state.updating ? ['text-gray', 'opacity-50'] : state.failed ? ['text-red'] : []">
            <span :class="modifying ? ['bg-white', 'text-black', 'rounded'] : []">
                <span v-for="value in [2, 4, 6, 8, 10]">
                    <span v-if="state.current !== null && state.current >= value">
                        &starf;
                    </span>
                    <span v-else>
                        &star;
                    </span>
                </span>
            </span>
        </div>
    </NavigableItem>
</template>
