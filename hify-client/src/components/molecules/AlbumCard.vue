<script setup lang="ts">
import type { AlbumFragment } from '@/graphql/generated/graphql'
import router from '@/router';
import Card from './Card.vue';
import { getAlbumArtUrl } from '@/global/constants';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';

defineProps<{
    album: AlbumFragment,
}>()
</script>

<template>
    <NavigableItem @press="router.push({ name: 'album', params: { id: album.id } })">
        <Card :title="album.name" :subtitle="album.albumArtists.map((artist) => artist.name).join(', ')"
            :art-url="getAlbumArtUrl(album)" />
    </NavigableItem>
</template>
