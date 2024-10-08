<script setup lang="ts">
import DropdownSelect, { type DropdownSelectExposeType, type DropdownChoices } from '@/components/molecules/DropdownSelect.vue';
import { ref } from 'vue';
import AlbumsFetcher from './AlbumsPage/AlbumsFetcher.vue';
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue';
import AlbumCard from '@/components/molecules/AlbumCard.vue';
import { isApproachingGridEnd } from '@/global/utils';
import { GRID_ALBUMS_PER_ROW } from '@/global/constants';

const sortByItems: DropdownChoices<'name' | 'date'> = [
  { id: 'name', label: 'Name' },
  { id: 'date', label: 'Last update date' },
]

const sortBy = ref<typeof sortByItems[number]['id']>('name')

const dropdownRef = ref<DropdownSelectExposeType | null>(null)
</script>

<template>
  <DropdownSelect ref="dropdownRef" prefix-label="Sort by:" :items="sortByItems" v-model="sortBy" />

  <AlbumsFetcher :sort-by="sortBy" v-slot="{ albums, feedMore }">
    <NavigableGrid :columns="GRID_ALBUMS_PER_ROW">
      <AlbumCard v-for="album, i in albums" :key="album.id" :album
        @focus="isApproachingGridEnd(i, GRID_ALBUMS_PER_ROW, albums.length) && feedMore()" />
    </NavigableGrid>
  </AlbumsFetcher>
</template>
