<script setup lang="ts">
import TracksGrid from '@/components/molecules/TracksGrid.vue';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { noParallel } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { AudioTrackFragment, HistoryPageQuery } from '@/graphql/generated/graphql';
import { onMounted, ref } from 'vue';

const TRACKS_PER_LINE = 6
const LINES_PER_PAGE = 5

const feedMore = noParallel(async () => {
  if (currentPageInfo.value?.hasNextPage === false) {
    return
  }

  const { data, error } = await gqlClient.query(
    graphql(`
      query HistoryPage($pagination: PaginationInput!) {
        history(pagination: $pagination) {
          nodes {
            ...AudioTrack
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
        first: TRACKS_PER_LINE * LINES_PER_PAGE
      }
    }
  )

  if (!data) {
    logFatal('Failed to fetch albums list', error)
  }

  currentPageInfo.value = data.history.pageInfo
  tracks.value.push(...data.history.nodes)
})

const currentPageInfo = ref<HistoryPageQuery['history']['pageInfo'] | null>(null)

const tracks = ref<AudioTrackFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <TracksGrid :tracks :feed-more />
</template>
