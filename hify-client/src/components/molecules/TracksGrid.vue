<script setup lang="ts">
import type { EntryInPlaylist } from '@/global/context-menu-items';
import { GRID_TRACKS_PER_ROW } from '@/global/constants';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import TrackCard from './TrackCard.vue';
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue';

defineProps<{
    tracks: AudioTrackFragment[]
    inPlaylist?: Omit<EntryInPlaylist, 'trackEntry'>
}>()

defineEmits<{
    feedMore: []
}>()
</script>

<template>
    <!-- TODO: implement "lazy-loader" attr -->
    <NavigableGrid :columns="GRID_TRACKS_PER_ROW" :lazy-loader="() => $emit('feedMore')">
        <TrackCard v-for="track, i in tracks" :track :tracks
            :in-playlist="inPlaylist && { ...inPlaylist, trackEntry: inPlaylist.allEntries[i] }" />
    </NavigableGrid>
</template>