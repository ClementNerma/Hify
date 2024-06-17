<script setup lang="ts">
import { MixOrdering, type ArtistFragment } from '@/graphql/generated/graphql'
import { type ContextMenuOption, showContextMenu } from '@/navigable/ui/molecules/ContextMenu/ContextMenu'
import { MIN_GREAT_RATING } from '@/global/constants'
import { generateAndPlayMix } from '@/global/stores/play-queue'
import { computed } from 'vue'
import router from '@/router'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import Card from './Card.vue'

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
    <SimpleNavigableItem @press="router.push({ name: 'artist', params: { id: artist.id } })"
        @long-press="showContextMenu(contextMenuOptions)">
        <Card :title="artist.name" art-type="artist" :art-item="artist" circle />
    </SimpleNavigableItem>
</template>
