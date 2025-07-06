<script setup lang="ts">
import { MIN_GREAT_RATING, getArtistArtUrl } from '@/global/constants'
import { type ContextMenuOption, showContextMenu } from '@/global/stores/context-menu'
import { generateAndPlayMix } from '@/global/stores/play-queue'
import { type ArtistFragment, MixOrdering } from '@/graphql/generated/graphql'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'
import router from '@/router'
import { computed } from 'vue'
import Card from './Card.vue'

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
