<script setup lang="ts">
import { computed } from 'vue'
import { getArtistArtUrl, MIN_GREAT_RATING } from '@/global/constants'
import { type ContextMenuOption, showContextMenu } from '@/global/stores/context-menu'
import { generateAndPlayMix } from '@/global/stores/play-queue'
import { type ArtistFragment, MixOrdering } from '@/graphql/generated/graphql'
import NavigableItem, { type NavigableItemProps } from '@/navigable/vue/components/NavigableItem.vue'
import router from '@/router'
import Card from './Card.vue'

const { artist, ...navigableItemProps } = defineProps<{
	artist: ArtistFragment
} & NavigableItemProps>()

const contextMenuOptions = computed<ContextMenuOption[]>(() => [
	{
		label: 'Mix me some magic ✨',
		onPress() {
			generateAndPlayMix({
				source: { artists: [artist.id] },
				ordering: MixOrdering.Random,
				minRating: MIN_GREAT_RATING,
			})
		},
	},
])
</script>

<template>
	<NavigableItem
		@press="router.push({ name: 'artist', params: { id: artist.id } })"
		@long-press="showContextMenu(contextMenuOptions)"
		v-bind="navigableItemProps"
	>
		<Card
			:title="artist.name"
			:art-url="getArtistArtUrl(artist, 'small')"
			circle
		/>
	</NavigableItem>
</template>
