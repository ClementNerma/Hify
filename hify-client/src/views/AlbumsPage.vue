<script setup lang="ts">
import AlbumCard from '@/components/molecules/AlbumCard.vue';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { isApproachingGridEnd, noParallel } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { AlbumFragment, AlbumsPageQuery } from '@/graphql/generated/graphql';
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue';
import { onMounted, ref } from 'vue';

const ALBUMS_PER_LINE = 6
const LINES_PER_PAGE = 5

const feedMore = noParallel(async () => {
  if (currentPageInfo.value?.hasNextPage === false) {
    return
  }

  const { data, error } = await gqlClient.query(
    graphql(`
      query AlbumsPage($pagination: PaginationInput!) {
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
        after: currentPageInfo.value?.endCursor,
        first: ALBUMS_PER_LINE * LINES_PER_PAGE
      }
    }
  )

  if (!data) {
    logFatal('Failed to fetch albums list', error)
  }

  currentPageInfo.value = data.albums.pageInfo
  albums.value.push(...data.albums.nodes)
})

const currentPageInfo = ref<AlbumsPageQuery['albums']['pageInfo'] | null>(null)

const albums = ref<AlbumFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <NavigableGrid :columns="ALBUMS_PER_LINE">
    <AlbumCard v-for="album, i in albums" :key="album.id" :album
      @focus="isApproachingGridEnd(i, ALBUMS_PER_LINE, albums.length) && feedMore()" />
  </NavigableGrid>
</template>
