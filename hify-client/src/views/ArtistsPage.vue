<script setup lang="ts">
import ArtistCard from '@/components/molecules/ArtistCard.vue';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { isApproachingGridEnd, noParallel } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { ArtistFragment, ArtistsPageQuery } from '@/graphql/generated/graphql';
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue';
import { onMounted, ref } from 'vue';

const ARTISTS_PER_LINE = 6
const LINES_PER_PAGE = 5

const feedMore = noParallel(async () => {
  if (currentPageInfo.value?.hasNextPage === false) {
    return
  }

  const { data, error } = await gqlClient.query(
    graphql(`
      query ArtistsPage($pagination: PaginationInput!) {
        artists(pagination: $pagination) {
          nodes {
            ...Artist
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
        first: ARTISTS_PER_LINE * LINES_PER_PAGE
      }
    }
  )

  if (!data) {
    logFatal('Failed to fetch albums list', error)
  }

  currentPageInfo.value = data.artists.pageInfo
  artists.value.push(...data.artists.nodes)
})

const currentPageInfo = ref<ArtistsPageQuery['artists']['pageInfo'] | null>(null)

const artists = ref<ArtistFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <NavigableGrid :columns="ARTISTS_PER_LINE">
    <ArtistCard v-for="artist, i in artists" :key="artist.id" :artist
      @focus="isApproachingGridEnd(i, ARTISTS_PER_LINE, artists.length) && feedMore()" />
  </NavigableGrid>
</template>