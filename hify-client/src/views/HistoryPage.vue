<script setup lang="ts">
import { onMounted, ref } from 'vue'
import TracksGrid from '@/components/molecules/TracksGrid.vue'
import { GRID_TRACKS_PER_ROW, GRID_TRACKS_PRELOAD_ROWS } from '@/global/constants'
import { gqlClient } from '@/global/urql-client'
import { noParallel } from '@/global/utils'
import { graphql } from '@/graphql/generated'
import type { AudioTrackFragment, HistoryPageQuery } from '@/graphql/generated/graphql'
import { logFatal } from '@/navigable'

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
				first: GRID_TRACKS_PER_ROW * GRID_TRACKS_PRELOAD_ROWS,
			},
		},
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
	<TracksGrid
		:tracks
		@feed-more="feedMore"
	/>
</template>
