<script setup lang="ts">
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import TracksFromAlbums from '@/components/organisms/TracksFromAlbums.vue';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { getRouteParam, noParallel } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { ArtistAllTracksQuery, AudioTrackFragment } from '@/graphql/generated/graphql';
import { onMounted, ref } from 'vue';

const artistId = getRouteParam('id')

const TRACKS_PER_PAGE = 30

const feedMore = noParallel(async () => {
  if (currentPageInfo.value?.hasNextPage === false) {
    return
  }

  const { data, error } = await gqlClient.query(
    graphql(`
      query ArtistAllTracks($artistId: String!, $pagination: PaginationInput!) {
        artist(id: $artistId) {
          name
          
          allTracks(pagination: $pagination) {
            nodes {
              ...AudioTrack
            }

            pageInfo {
              endCursor
              hasNextPage
            }
          }
        }
      }
    `),
    {
      artistId,
      pagination: {
        after: currentPageInfo.value?.endCursor,
        first: TRACKS_PER_PAGE
      }
    }
  )

  if (!data?.artist) {
    logFatal('Failed to fetch track participations', error)
  }

  authorName.value = data.artist.name
  currentPageInfo.value = data.artist.allTracks.pageInfo
  tracks.value.push(...data.artist.allTracks.nodes)
})

const currentPageInfo = ref<NonNullable<ArtistAllTracksQuery['artist']>['allTracks']['pageInfo'] | null>(null)
const authorName = ref<string | null>(null)

const tracks = ref<AudioTrackFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <LoadingIndicator v-if="!currentPageInfo || !authorName" :error="null /* TODO */" />

  <template v-else-if="tracks.length > 0">
    <h1>All tracks from {{ authorName }} ({{ tracks.length }})</h1>

    <TracksFromAlbums :tracks :has-more="currentPageInfo.hasNextPage" :feed-more />
  </template>
</template>
