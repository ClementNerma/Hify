<script setup lang="ts">
import type { AlbumFragment } from '@/graphql/generated/graphql'
import type { SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import InteractiveCard from './InteractiveCard.vue'
import router from '@/router';

defineProps<{
    album: AlbumFragment,
    onFocus?: SimpleNavigableItemProps['onFocus']
}>()
</script>

<template>
    <InteractiveCard :title="album.name" :subtitle="album.albumArtists.map((artist) => artist.name).join(', ')"
        @press="router.push({ name: 'album', params: { id: album.id } })" :art="album.art" :on-focus />
</template>
