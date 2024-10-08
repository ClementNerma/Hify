<script setup lang="ts">
import type { AlbumFragment } from '@/graphql/generated/graphql';
import AlbumsByNameFetcher from './AlbumsByNameFetcher.vue';
import AlbumsByDateFetcher from './AlbumsByDateFetcher.vue';

defineProps<{
  sortBy: 'name' | 'date'
}>()

defineSlots<{
  default(props: { albums: AlbumFragment[], feedMore: () => void }): unknown
}>()
</script>

<template>
  <AlbumsByNameFetcher v-slot="slot" v-if="sortBy === 'name'">
    <slot v-bind="slot" />
  </AlbumsByNameFetcher>

  <AlbumsByDateFetcher v-slot="slot" v-else-if="sortBy === 'date'">
    <slot v-bind="slot" />
  </AlbumsByDateFetcher>
</template>
