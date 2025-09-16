<script setup lang="ts">
import { getAlbumArtUrl } from '@/global/constants'
import type { AlbumFragment } from '@/graphql/generated/graphql'
import NavigableItem, { type NavigableItemProps } from '@/navigable/vue/components/NavigableItem.vue'
import router from '@/router'
import Card from './Card.vue'

const props = defineProps<
    {
        album: AlbumFragment
    } & NavigableItemProps
>()
</script>

<template>
    <NavigableItem
        v-bind="props"
        @press="router.push({ name: 'album', params: { id: album.id } })"
    >
        <Card
            :title="album.name"
            :subtitle="album.albumArtists.map((artist) => artist.name).join(', ')"
            :art-url="getAlbumArtUrl(album, 'small')"
        />
    </NavigableItem>
</template>
