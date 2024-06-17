<script setup lang="ts">
import type { AlbumFragment } from '@/graphql/generated/graphql'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';
import router from '@/router';
import Card from './Card.vue';

defineProps<{
    album: AlbumFragment,
}>()
</script>

<template>
    <SimpleNavigableItem @press="router.push({ name: 'album', params: { id: album.id } })">
        <Card :title="album.name" :subtitle="album.albumArtists.map((artist) => artist.name).join(', ')"
            art-type="album" :art-item="album" />
    </SimpleNavigableItem>
</template>
