<script setup lang="ts">
import { MixOrdering, type ArtistFragment } from '@/graphql/generated/graphql'
import { MIN_GREAT_RATING, getArtistArtUrl } from '@/global/constants'
import { generateAndPlayMix } from '@/global/stores/play-queue'
import { computed } from 'vue'
import router from '@/router'
import Card from './Card.vue'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'
import { showContextMenu, type ContextMenuOption } from '@/global/stores/context-menu'

const { artist } = defineProps<{
	artist: ArtistFragment
}>()

const contextMenuOptions = computed<ContextMenuOption[]>(() => [
	{
		label: 'Mix me some magic ✨',
		onPress() {
			generateAndPlayMix({
				source: { allTracks: true },
				ordering: MixOrdering.Random,
				minRating: MIN_GREAT_RATING,
				fromArtists: [artist.id],
			})
		},
	},
])
</script>

<template>
    <NavigableItem @press="router.push({ name: 'artist', params: { id: artist.id } })"
        @long-press="showContextMenu(contextMenuOptions)">
        <Card :title="artist.name" :art-url="getArtistArtUrl(artist)" circle />
    </NavigableItem>
</template>
