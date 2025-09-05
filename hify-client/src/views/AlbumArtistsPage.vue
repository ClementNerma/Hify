<script setup lang="ts">
import { onMounted, ref } from 'vue'
import ArtistCard from '@/components/molecules/ArtistCard.vue'
import { gqlClient } from '@/global/urql-client'
import { isApproachingGridEnd, noParallel } from '@/global/utils'
import { graphql } from '@/graphql/generated'
import type { AlbumArtistsPageQuery, ArtistFragment } from '@/graphql/generated/graphql'
import { logFatal } from '@/navigable'
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue'

const ARTISTS_PER_LINE = 6
const LINES_PER_PAGE = 5

const feedMore = noParallel(async () => {
	if (currentPageInfo.value?.hasNextPage === false) {
		return
	}

	const { data, error } = await gqlClient.query(
		graphql(`
      query AlbumArtistsPage($pagination: PaginationInput!) {
        albumArtists(pagination: $pagination) {
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
				first: ARTISTS_PER_LINE * LINES_PER_PAGE,
			},
		},
	)

	if (!data) {
		logFatal('Failed to fetch albums list', error)
	}

	currentPageInfo.value = data.albumArtists.pageInfo
	artists.value.push(...data.albumArtists.nodes)
})

const currentPageInfo = ref<AlbumArtistsPageQuery['albumArtists']['pageInfo'] | null>(null)

const artists = ref<ArtistFragment[]>([])

onMounted(feedMore)
</script>

<template>
	<NavigableGrid :columns="ARTISTS_PER_LINE">
		<ArtistCard
			v-for="artist, i in artists"
			:key="artist.id"
			:artist
			@focus="isApproachingGridEnd(i, ARTISTS_PER_LINE, artists.length) && feedMore()"
		/>
	</NavigableGrid>
</template>