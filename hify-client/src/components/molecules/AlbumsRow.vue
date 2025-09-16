<script setup lang="ts">
import { getAlbumArtUrl } from '@/global/constants'
import type { AlbumFragment } from '@/graphql/generated/graphql'
import router from '@/router'
import Card from './Card.vue'
import ProgressiveRow from './ProgressiveRow.vue'

defineProps<{
    albums: AlbumFragment[]
}>()
</script>

<template>
    <ProgressiveRow
        :items="albums"
        idProp="id"
        @item-press="(album) => router.push({ name: 'album', params: { id: album.id } })"
        v-slot="{ item: album }"
    >
        <Card
            :title="album.name"
            :subtitle="album.albumArtists.map((artist) => artist.name).join(', ')"
            :art-url="getAlbumArtUrl(album, 'small')"
        />
    </ProgressiveRow>
</template>