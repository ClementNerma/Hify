<script setup lang="ts">
import Button from '@/components/atoms/Button.vue';
import Checkbox from '@/components/atoms/Checkbox.vue';
import Emoji from '@/components/atoms/Emoji.vue';
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import TracksFromAlbums from '@/components/organisms/TracksFromAlbums.vue';
import { showContextMenu } from '@/global/stores/context-menu';
import { logFatal } from '@/navigable';
import { enqueue, playNewQueueFromBeginning } from '@/global/stores/play-queue';
import { gqlClient } from '@/global/urql-client';
import { getRouteParam, hasMinimumRating, noParallel, shuffle } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { ArtistAllTracksQuery, AudioTrackFragment } from '@/graphql/generated/graphql';
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue';
import router from '@/router';
import { computed, onMounted, ref } from 'vue';

const artistId = getRouteParam('id')

const TRACKS_PER_PAGE = 100

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
  unfilteredTracks.value.push(...data.artist.allTracks.nodes)
})

const currentPageInfo = ref<NonNullable<ArtistAllTracksQuery['artist']>['allTracks']['pageInfo'] | null>(null)
const authorName = ref<string | null>(null)

const unfilteredTracks = ref<AudioTrackFragment[]>([])

const onlyShowGreatSongs = ref(false)
const filteredTracks = computed(() => onlyShowGreatSongs.value ? unfilteredTracks.value.filter((track) => hasMinimumRating(track, 8)) : unfilteredTracks.value)

onMounted(feedMore)
</script>

<template>
  <LoadingIndicator v-if="!currentPageInfo || !authorName" :error="null /* TODO */" />

  <div class="mt-2.5" v-else-if="unfilteredTracks.length > 0">
    <h1>All tracks from {{ authorName }}</h1>

    <div class="flex flex-row items-center">
      <NavigableRow>
        <Checkbox v-model="onlyShowGreatSongs">Only show great songs</Checkbox>

        <Button @press="enqueue(filteredTracks, 'next')" @long-press="showContextMenu([
          {
            label: 'Queue at the end',
            onPress: () => enqueue(filteredTracks, 'end'),
          }
        ])">
          <Emoji>▶️</Emoji> Play next
        </Button>

        <Button @press="() => {
          playNewQueueFromBeginning(shuffle(filteredTracks), null)
          router.push({ name: 'now-playing' })
        }">
          <Emoji>🔀</Emoji> Shuffle
        </Button>
      </NavigableRow>
    </div>

    <TracksFromAlbums :tracks="filteredTracks" :has-more="currentPageInfo.hasNextPage" :feed-more />
  </div>
</template>
