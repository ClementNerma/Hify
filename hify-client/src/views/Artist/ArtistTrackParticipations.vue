<script setup lang="ts">
import Centered from '@/components/atoms/Centered.vue'
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue'
import TracksFromAlbums from '@/components/organisms/TracksFromAlbums.vue'
import { gqlClient } from '@/global/urql-client'
import { noParallel } from '@/global/utils'
import { graphql } from '@/graphql/generated'
import type { ArtistTrackParticipationsQuery, AudioTrackFragment } from '@/graphql/generated/graphql'
import { logFatal } from '@/navigable'
import { onMounted, ref } from 'vue'

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
				first: TRACKS_PER_LINE * LINES_PER_PAGE,
			},
		},
	)

	if (!data?.artist) {
		logFatal('Failed to fetch track participations', error)
	}

	currentPageInfo.value = data.artist.trackParticipations.pageInfo
	tracks.value.push(...data.artist.trackParticipations.nodes)
})

const currentPageInfo = ref<
	NonNullable<ArtistTrackParticipationsQuery['artist']>['trackParticipations']['pageInfo'] | null
>(null)

const tracks = ref<AudioTrackFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <LoadingIndicator v-if="!currentPageInfo" :error="null /* TODO */" />

  <template v-else-if="tracks.length > 0">
    <Centered>
      <h3>Tracks from other artists' albums ({{ tracks.length }})</h3>
    </Centered>

    <TracksFromAlbums :tracks :has-more="currentPageInfo.hasNextPage" :feed-more show-artists-name />
  </template>
</template>
