<script setup lang="ts">
import Button from '@/components/atoms/Button.vue';
import ImgLoader from '@/components/atoms/ImgLoader.vue';
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import NavigableTrack from '@/components/atoms/NavigableTrack.vue';
import TrackRating from '@/components/atoms/TrackRating.vue';
import { humanReadableDuration } from '@/global/stores/audio-player';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { graphql } from '@/graphql/generated';
import type { ArtistTrackParticipationsQuery, AudioTrackFragment } from '@/graphql/generated/graphql';
import NavigableList from '@/navigable/headless/NavigableList/NavigableList.vue';
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


    <table class="mt-2.5 w-1/2 border-collapse">
      <tbody>
        <NavigableList>
          <NavigableTrack v-for="track, i in tracks" :key="track.id" :tracks :context="{ context: 'album' }" :track>
            <tr class="w-full [&>td]:p-2.5" :class="i > 0 ? ['border-0 border-t border-solid border-gray-700'] : []">
              <!-- TODO: show album title + album art -->
              <td>{{ track.metadata.tags.trackNo }}</td>
              <td class="w-full">{{ track.metadata.tags.title }}</td>
              <td>
                <span v-if="track.computedRating">
                  <TrackRating :rating="track.computedRating" />
                </span>
              </td>
              <td class="text-right">{{ humanReadableDuration(track.metadata.duration) }}</td>
            </tr>
          </NavigableTrack>
        </NavigableList>
      </tbody>
    </table>

    <Button v-if="currentPageInfo.hasNextPage" @press="feedMore()">
      Load more
    </Button>

  </template>
</template>
