<script setup lang="ts">
import NavigableTrack from '../atoms/NavigableTrack.vue'
import type { EntryInPlaylist } from '@/global/context-menu-items'
import Card from './Card.vue'
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import { getAlbumArtUrl } from '@/global/constants';

defineProps<{
    track: AudioTrackFragment,
    tracks: AudioTrackFragment[],
    inPlaylist?: EntryInPlaylist,
    onFocus?: () => void
}>()

</script>

<template>
    <NavigableTrack :track :tracks :context="inPlaylist ? { context: 'playlist', entry: inPlaylist } : {
        context: 'normal'
    }" :on-focus>
        <Card :title="track.metadata.tags.title" :art-url="getAlbumArtUrl(track.metadata.tags.album)" />
    </NavigableTrack>
</template>