<script setup lang="ts">
import { GRID_TRACKS_PER_ROW } from '@/global/constants'
import type { EntryInPlaylist } from '@/global/ctx-menu-content'
import { isApproachingGridEnd } from '@/global/utils'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue'
import TrackCard from './TrackCard.vue'

defineProps<{
	tracks: AudioTrackFragment[]
	inPlaylist?: Omit<EntryInPlaylist, 'trackEntry'>
}>()

defineEmits<{
	feedMore: []
}>()
</script>

<template>
    <NavigableGrid :columns="GRID_TRACKS_PER_ROW">
        <TrackCard v-for="track, i in tracks" :track :tracks
            :in-playlist="inPlaylist && { ...inPlaylist, trackEntry: inPlaylist.allEntries[i] }"
            @focus="isApproachingGridEnd(i, GRID_TRACKS_PER_ROW, tracks.length) && $emit('feedMore')" />
    </NavigableGrid>
</template>
