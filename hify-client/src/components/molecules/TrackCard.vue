<script setup lang="ts">
import { getAlbumArtUrl } from '@/global/constants'
import type { EntryInPlaylist } from '@/global/ctx-menu-content'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import NavigableTrack from '../atoms/NavigableTrack.vue'
import Card from './Card.vue'

defineProps<{
    track: AudioTrackFragment
    tracks: AudioTrackFragment[]
    inPlaylist?: EntryInPlaylist
    onFocus?: () => void
}>()
</script>

<template>
    <NavigableTrack
        :track
        :tracks
        :context="inPlaylist ? { context: 'playlist', entry: inPlaylist } : {
            context: 'normal'
        }"
        :on-focus
    >
        <Card
            :title="track.metadata.tags.title"
            :art-url="getAlbumArtUrl(track.metadata.tags.album, 'small')"
        />
    </NavigableTrack>
</template>