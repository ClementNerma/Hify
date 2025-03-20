<script setup lang="ts">
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue'
import MixButton from '@/components/atoms/MixButton.vue'
import AlbumCard from '@/components/molecules/AlbumCard.vue'
import { MIN_GREAT_RATING } from '@/global/constants'
import { logFatal } from '@/navigable'
import { gqlClient } from '@/global/urql-client'
import { getRouteParam, isApproachingGridEnd, noParallel } from '@/global/utils'
import { graphql } from '@/graphql/generated'
import { MixOrdering, type AlbumFragment, type GenrePageQuery } from '@/graphql/generated/graphql'
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue'
import { onMounted, ref } from 'vue'

const ALBUMS_PER_LINE = 6
const LINES_PER_PAGE = 5

const genreId = getRouteParam('id')

const feedMore = noParallel(async () => {
	if (currentPageInfo.value?.hasNextPage === false) {
		return
	}

	const { data, error } = await gqlClient.query(
		graphql(`
      query GenrePage($genreId: String!, $pagination: PaginationInput!) {
        genre(id: $genreId) {
          name

          albums(pagination: $pagination) {
            nodes {
              ...Album
            }

            pageInfo {
              hasNextPage
              endCursor
            }
          }
        }
      }
    `),
		{
			genreId,
			pagination: {
				after: currentPageInfo.value?.endCursor,
				first: ALBUMS_PER_LINE * LINES_PER_PAGE,
			},
		},
	)

	if (!data?.genre) {
		logFatal('Failed to fetch albums list', error)
	}

	currentPageInfo.value = data.genre.albums.pageInfo
	albums.value.push(...data.genre.albums.nodes)
	genreName.value = data.genre.name
})

const currentPageInfo = ref<NonNullable<GenrePageQuery['genre']>['albums']['pageInfo'] | null>(null)
const genreName = ref<string | null>(null)
const albums = ref<AlbumFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <LoadingIndicator v-if="!genreName" :error="null /* TODO */" />

  <template v-else>
    <h2>Genre: {{ genreName }}</h2>

    <MixButton :mixParams="{
      source: { allTracks: true },
      ordering: MixOrdering.Random,
      minRating: MIN_GREAT_RATING,
      fromGenres: [genreId],
    }" />

    <h3>List of albums</h3>

    <NavigableGrid :columns="ALBUMS_PER_LINE">
      <AlbumCard v-for="album, i in albums" :key="album.id" :album
        @focus="isApproachingGridEnd(i, ALBUMS_PER_LINE, albums.length) && feedMore()" />
    </NavigableGrid>
  </template>
</template>
