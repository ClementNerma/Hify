<script setup lang="ts">
import { GRID_ALBUMS_PER_ROW, GRID_ALBUMS_PRELOAD_ROWS } from '@/global/constants';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { noParallel } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { AlbumFragment, MostRecentAlbumsQuery } from '@/graphql/generated/graphql';
import { onMounted, ref } from 'vue';

const feedMore = noParallel(async () => {
  if (currentPageInfo.value?.hasNextPage === false) {
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
        after: currentPageInfo.value?.endCursor,
        first: GRID_ALBUMS_PER_ROW * GRID_ALBUMS_PRELOAD_ROWS
      }
    }
  )

  if (!data) {
    logFatal('Failed to fetch albums list', error)
  }

  currentPageInfo.value = data.mostRecentAlbums.pageInfo
  albums.value.push(...data.mostRecentAlbums.nodes)
})

const currentPageInfo = ref<MostRecentAlbumsQuery['mostRecentAlbums']['pageInfo'] | null>(null)

const albums = ref<AlbumFragment[]>([])

onMounted(feedMore)

defineSlots<{
  default(props: { albums: AlbumFragment[], feedMore: () => void }): unknown
}>()
</script>

<template>
  <slot :albums :feed-more />
</template>