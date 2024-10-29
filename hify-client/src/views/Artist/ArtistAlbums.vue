<script setup lang="ts">
import Centered from '@/components/atoms/Centered.vue';
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import AlbumCard from '@/components/molecules/AlbumCard.vue';
import { logFatal } from '@/navigable';
import { gqlClient } from '@/global/urql-client';
import { isApproachingGridEnd, noParallel } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { AlbumFragment, ArtistAlbumsQuery } from '@/graphql/generated/graphql';
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue';
import { onMounted, ref } from 'vue';

const { artistId } = defineProps<{ artistId: string }>()

const ALBUMS_PER_LINE = 6
const LINES_PER_PAGE = 5

const feedMore = noParallel(async () => {
  if (currentPageInfo.value?.hasNextPage === false) {
    return
  }

  const { data, error } = await gqlClient.query(
    graphql(`
      query ArtistAlbums($artistId: String!, $pagination: PaginationInput!) {
        artist(id: $artistId) {
          albums(pagination: $pagination) {
            nodes {
              ...Album
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
        first: ALBUMS_PER_LINE * LINES_PER_PAGE
      }
    }
  )

  if (!data?.artist) {
    logFatal('Failed to fetch albums list', error)
  }

  currentPageInfo.value = data.artist.albums.pageInfo
  albums.value.push(...data.artist.albums.nodes)
})

const currentPageInfo = ref<NonNullable<ArtistAlbumsQuery['artist']>['albums']['pageInfo'] | null>(null)

const albums = ref<AlbumFragment[]>([])

onMounted(feedMore)
</script>

<template>
  <LoadingIndicator v-if="!currentPageInfo" :error="null /* TODO */" />

  <h3 v-else-if="albums.length === 0">No album</h3>

  <template v-else>
    <Centered>
      <h3>Albums ({{ albums.length }})</h3>
    </Centered>

    <NavigableGrid :columns="ALBUMS_PER_LINE">
      <AlbumCard v-for="album, i in albums" :key="album.id" :album enforce-max-width
        @focus="isApproachingGridEnd(i, ALBUMS_PER_LINE, albums.length) && feedMore()" />
    </NavigableGrid>
  </template>
</template>
