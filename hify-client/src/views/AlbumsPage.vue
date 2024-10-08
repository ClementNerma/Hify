<script setup lang="ts">
import AlbumCard from '@/components/molecules/AlbumCard.vue';
import DropdownSelect, { type DropdownSelectExposeType, type DropdownChoices } from '@/components/molecules/DropdownSelect.vue';
import { GRID_ALBUMS_PER_ROW, GRID_ALBUMS_PRELOAD_ROWS } from '@/global/constants';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { assertUnreachable, isApproachingGridEnd, noParallel } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { AlbumFragment, AlbumsByNameQuery, MostRecentAlbumsQuery } from '@/graphql/generated/graphql';
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue';
import { onMounted, ref, watch } from 'vue';

const feedMore = noParallel(async () => {
  switch (sortBy.value) {
    case 'name': {
      if (fetchState.value.sortBy === 'name' && fetchState.value.pageInfo?.hasNextPage === false) {
        return
      }

      const { data, error } = await gqlClient.query(
        graphql(`
          query AlbumsByName($pagination: PaginationInput!) {
            albums(pagination: $pagination) {
              nodes {
                ...Album
              }

              pageInfo {
                endCursor
                hasNextPage
              }
            }
          }
        `),
        {
          pagination: {
            after: fetchState.value.sortBy === 'name' ? fetchState.value.pageInfo?.endCursor : null,
            first: GRID_ALBUMS_PER_ROW * GRID_ALBUMS_PRELOAD_ROWS
          }
        }
      )

      if (!data) {
        logFatal('Failed to fetch albums list', error)
      }

      if (fetchState.value.sortBy === 'name') {
        albums.value.push(...data.albums.nodes)
      } else {
        albums.value = data.albums.nodes
      }

      fetchState.value = { sortBy: 'name', pageInfo: data.albums.pageInfo }

      break
    }

    case 'date': {
      if (fetchState.value.sortBy === 'date' && fetchState.value.pageInfo?.hasNextPage === false) {
        return
      }

      const { data, error } = await gqlClient.query(
        graphql(`
          query MostRecentAlbums($pagination: PaginationInput!) {
            mostRecentAlbums(pagination: $pagination) {
              nodes {
                ...Album
              }

              pageInfo {
                endCursor
                hasNextPage
              }
            }
          }
        `),
        {
          pagination: {
            after: fetchState.value.sortBy === 'date' ? fetchState.value.pageInfo?.endCursor : null,
            first: GRID_ALBUMS_PER_ROW * GRID_ALBUMS_PRELOAD_ROWS
          }
        }
      )

      if (!data) {
        logFatal('Failed to fetch albums list', error)
      }

      if (fetchState.value.sortBy === 'date') {
        albums.value.push(...data.mostRecentAlbums.nodes)
      } else {
        albums.value = data.mostRecentAlbums.nodes
      }

      fetchState.value = { sortBy: 'date', pageInfo: data.mostRecentAlbums.pageInfo }

      break
    }

    default:
      assertUnreachable(sortBy.value)
  }
})

type SortBy = 'name' | 'date'

const sortByItems: DropdownChoices<SortBy> = [
  { id: 'name', label: 'Name' },
  { id: 'date', label: 'Last update date' },
]

const fetchState = ref<
  { sortBy: 'name', pageInfo: AlbumsByNameQuery['albums']['pageInfo'] | null } |
  { sortBy: 'date', pageInfo: MostRecentAlbumsQuery['mostRecentAlbums']['pageInfo'] | null }
>({ sortBy: 'name', pageInfo: null })

const albums = ref<AlbumFragment[]>([])

const sortBy = ref<SortBy>('name')

const dropdownRef = ref<DropdownSelectExposeType | null>(null)

onMounted(() => {
  feedMore()

  watch(sortBy, () => {
    feedMore()
  })
})
</script>

<template>
  <DropdownSelect ref="dropdownRef" prefix-label="Sort by:" :items="sortByItems" v-model="sortBy" />

  <NavigableGrid :columns="GRID_ALBUMS_PER_ROW">
    <AlbumCard v-for="album, i in albums" :key="`${sortBy}/${album.id}`" :album
      @focus="isApproachingGridEnd(i, GRID_ALBUMS_PER_ROW, albums.length) && feedMore()" />
  </NavigableGrid>
</template>
