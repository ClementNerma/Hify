<script setup lang="ts">
import Button from '@/components/atoms/Button.vue';
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import AlbumCard from '@/components/molecules/AlbumCard.vue';
import TrackCard from '@/components/molecules/TrackCard.vue';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { graphql } from '@/graphql/generated';
import type { AlbumFragment, AlbumsPageQuery, ArtistAlbumsQuery, ArtistPageQuery, ArtistTrackParticipationsQuery, AudioTrackFragment } from '@/graphql/generated/graphql';
import Grid from '@/navigable/ui/organisms/Grid.vue';
import { onMounted, ref } from 'vue';

const { artistId } = defineProps<{ artistId: string }>()

const TRACKS_PER_LINE = 6
const LINES_PER_PAGE = 5

async function feedMore() {
  if (currentPageInfo.value?.hasNextPage === false) {
    return
  }

  const { data, error } = await gqlClient.query(
    graphql(`
      query ArtistTrackParticipations($artistId: String!, $pagination: PaginationInput!) {
        artist(id: $artistId) {
          trackParticipations(pagination: $pagination) {
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
        first: TRACKS_PER_LINE * LINES_PER_PAGE
      }
    }
  )

  if (!data?.artist) {
    logFatal('Failed to fetch track participations', error)
  }

  currentPageInfo.value = data.artist.trackParticipations.pageInfo
  tracks.value.push(...data.artist.trackParticipations.nodes)
}

const currentPageInfo = ref<NonNullable<ArtistTrackParticipationsQuery['artist']>['trackParticipations']['pageInfo'] | null>(null)

const tracks = ref<AudioTrackFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <LoadingIndicator v-if="!currentPageInfo" />

  <template v-else-if="tracks.length > 0">
    <h3>Tracks from other artists' albums ({{ tracks.length }})</h3>

    <Grid :columns="TRACKS_PER_LINE">
      <TrackCard v-for="track in tracks" :key="track.id" :track :tracks />
    </Grid>

    <Button v-if="currentPageInfo.hasNextPage" @press="feedMore()">
      Load more
    </Button>
  </template>
</template>
