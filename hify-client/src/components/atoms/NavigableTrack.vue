<script setup lang="ts">
import { ctxMenuCallbacks, ctxMenuOptions, type TrackContext } from '@/global/ctx-menu-content'
import { showContextMenu } from '@/global/stores/context-menu'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import type { NavigableItem as NavigableItemType } from '@/navigable'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'

defineProps<{
	track: AudioTrackFragment
	tracks: AudioTrackFragment[]
	context: TrackContext
	fromMixId?: string
	onFocus?: () => void
}>()

defineSlots<{
	default(props: {
		item: NavigableItemType
		focused: boolean
	}): unknown
}>()
</script>

<template>
    <NavigableItem v-slot="{ item, focused }" @press="ctxMenuCallbacks.playTrack(track, tracks, fromMixId ?? null)"
        @long-press="showContextMenu(ctxMenuOptions.forTrack(track, { fromMixId: fromMixId ?? null }, context))"
        :on-focus>
        <slot :item :focused />
    </NavigableItem>
</template>