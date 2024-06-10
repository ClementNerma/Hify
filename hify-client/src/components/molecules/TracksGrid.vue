<script setup lang="ts">
import type { EntryInPlaylist } from '@/global/context-menu-items';
import { GRID_TRACKS_PER_ROW } from '@/global/constants';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import Grid from '@/navigable/ui/organisms/Grid.vue';
import TrackCard from './TrackCard.vue';

defineProps<{
    tracks: AudioTrackFragment[]
    inPlaylist?: Omit<EntryInPlaylist, 'trackEntry'>
}>()

defineEmits<{
    feedMore: []
}>()
</script>

<template>
    <Grid :columns="GRID_TRACKS_PER_ROW" :lazy-loader="() => $emit('feedMore')">
        <TrackCard v-for="track, i in tracks" :track :tracks
            :in-playlist="inPlaylist && { ...inPlaylist, trackEntry: inPlaylist.allEntries[i] }" />
    </Grid>
</template>