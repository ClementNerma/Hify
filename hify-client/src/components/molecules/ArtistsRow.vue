<script setup lang="ts">
import router from '@/router'
import { ctxMenuOptions } from '@/global/ctx-menu-content'
import type { ArtistFragment } from '@/graphql/generated/graphql'
import Card from '@/components/molecules/Card.vue'
import ProgressiveRow from '@/components/molecules/ProgressiveRow.vue'
import { getArtistArtUrl } from '@/global/constants'
import { showContextMenu } from '@/global/stores/context-menu'

defineProps<{ artists: ArtistFragment[] }>()
</script>

<template>
    <ProgressiveRow :items="artists" idProp="id"
        @item-press="(artist) => router.push({ name: 'artist', params: { id: artist.id } })"
        @item-long-press="(artist) => showContextMenu(ctxMenuOptions.forArtist(artist.id))" v-slot="{ item: artist }">
        <Card :title="artist.name" :art-url="getArtistArtUrl(artist)" />
    </ProgressiveRow>
</template>
