<script setup lang="ts">
import { ArtFragmentDoc, type AlbumFragment } from '@/graphql/generated/graphql'
import Card from './Card.vue'
import ProgressiveRow from './ProgressiveRow.vue'
import router from '@/router';

defineProps<{
    albums: AlbumFragment[]
}>()
</script>

<template>
    <ProgressiveRow :items="albums" idProp="id"
        @item-press="(album) => router.push({ name: 'album', params: { id: album.id } })" v-slot="{ item: album }">
        <Card :title="album.name" :subtitle="album.albumArtists.map((artist) => artist.name).join(', ')"
            :art="album.art" />
    </ProgressiveRow>

</template>