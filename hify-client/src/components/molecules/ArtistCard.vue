<script setup lang="ts">
import { type ArtFragment, MixOrdering, type ArtistFragment, ArtFragmentDoc } from '@/graphql/generated/graphql'
import { type ContextMenuOption, showContextMenu } from '@/navigable/ui/molecules/ContextMenu/ContextMenu'
import { MIN_GREAT_RATING } from '@/global/constants'
import { generateAndPlayMix } from '@/global/stores/play-queue'
import InteractiveCard from './InteractiveCard.vue'
import { computed } from 'vue'
import router from '@/router'

const { artist } = defineProps<{
    artist: ArtistFragment
}>()

const contextMenuOptions = computed<ContextMenuOption[]>(() => [
    {
        label: 'Mix me some magic ✨',
        onPress() {
            generateAndPlayMix({
                source: { allTracks: '-' },
                ordering: MixOrdering.Random,
                minRating: MIN_GREAT_RATING,
                fromArtists: [artist.id]
            })
        }
    }

])
</script>

<template>
    <InteractiveCard :title="artist.name" @press="router.push({ name: 'artist', params: { id: artist.id } })"
        @long-press="showContextMenu(contextMenuOptions)" :art="artist.art" circle />
</template>
