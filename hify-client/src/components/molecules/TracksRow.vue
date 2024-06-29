<script setup lang="ts">
import { ctxMenuCallbacks, ctxMenuOptions } from '@/global/context-menu-items'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import Card from './Card.vue'
import ProgressiveRow from './ProgressiveRow.vue'
import { getAlbumArtUrl } from '@/global/constants'
import { showContextMenu } from '@/global/stores/context-menu'

defineProps<{ tracks: AudioTrackFragment[] }>() 
</script>

<template>
    <ProgressiveRow :items="tracks" idProp="id" @item-press="(track) => ctxMenuCallbacks.playTrack(track, tracks, null)"
        @item-long-press="(track) =>
            showContextMenu(ctxMenuOptions.forTrack(track, { fromMixId: null }, { context: 'normal' }))"
        v-slot="{ item: track }">
        <Card :title="track.metadata.tags.title" :art-url="getAlbumArtUrl(track.metadata.tags.album)" />
    </ProgressiveRow>
</template>