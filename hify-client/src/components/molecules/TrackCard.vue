<script setup lang="ts">
import NavigableTrack from '../atoms/NavigableTrack.vue'
import type { EntryInPlaylist } from '@/global/context-menu-items'
import Card from './Card.vue'
import type { SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem'
import type { AudioTrackFragment } from '@/graphql/generated/graphql';

defineProps<{
    track: AudioTrackFragment,
    tracks: AudioTrackFragment[],
    inPlaylist?: EntryInPlaylist,
    onFocus?: SimpleNavigableItemProps['onFocus']
}>()

</script>

<template>
    <NavigableTrack :track :tracks :context="inPlaylist ? { context: 'playlist', entry: inPlaylist } : {
        context: 'normal'
    }" :on-focus>
        <Card :title="track.metadata.tags.title" art-type="album" :art-item="track.metadata.tags.album" />
    </NavigableTrack>

</template>