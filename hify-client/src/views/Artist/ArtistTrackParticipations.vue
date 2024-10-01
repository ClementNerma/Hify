<script setup lang="ts">
import Button from '@/components/atoms/Button.vue';
import Centered from '@/components/atoms/Centered.vue';
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import NavigableTrack from '@/components/atoms/NavigableTrack.vue';
import TrackRating from '@/components/atoms/TrackRating.vue';
import { getAlbumArtUrl } from '@/global/constants';
import { humanReadableDuration } from '@/global/stores/audio-player';
import { logFatal } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { noParallel } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { ArtistTrackParticipationsQuery, AudioTrackFragment } from '@/graphql/generated/graphql';
import NavigableList from '@/navigable/vue/components/NavigableList.vue';
import { onMounted, ref } from 'vue';

const { artistId } = defineProps<{ artistId: string }>()

const TRACKS_PER_LINE = 6
const LINES_PER_PAGE = 5

const feedMore = noParallel(async () => {
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
})

const currentPageInfo = ref<NonNullable<ArtistTrackParticipationsQuery['artist']>['trackParticipations']['pageInfo'] | null>(null)

const tracks = ref<AudioTrackFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <LoadingIndicator v-if="!currentPageInfo" :error="null /* TODO */" />

  <template v-else-if="tracks.length > 0">
    <Centered>
      <h3>Tracks from other artists' albums ({{ tracks.length }})</h3>
    </Centered>

    <NavigableList>
      <table class="mt-2.5 w-3/4 border-collapse whitespace-nowrap">
        <tbody>
          <tr v-for="track, i in tracks" :key="track.id" class="[&>td]:p-2.5"
            :class="i > 0 ? ['border-0 border-t border-solid border-gray-700'] : []">
            <td>
              <img :width="50" :height="50" :src="getAlbumArtUrl(track.metadata.tags.album)" />
            </td>
            <td>
              {{ track.metadata.tags.album.name }}
              ({{ track.metadata.tags.album.albumArtists.map(artist => artist.name).join(', ') }})
            </td>
            <td class="w-full">
              <NavigableTrack :tracks :context="{ context: 'album' }" :track>
                <span>{{ track.metadata.tags.title }}</span>
              </NavigableTrack>
            </td>
            <td>
              <span v-if="track.computedRating">
                <TrackRating :rating="track.computedRating" />
              </span>
            </td>
            <td class="text-right">{{ humanReadableDuration(track.metadata.duration) }}</td>
          </tr>
        </tbody>
      </table>
    </NavigableList>

    <Button v-if="currentPageInfo.hasNextPage" @press="feedMore()">
      Load more
    </Button>

  </template>
</template>
