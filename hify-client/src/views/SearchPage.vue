<script setup lang="ts">
import Run from '@/components/atoms/Run.vue';
import AlbumsRow from '@/components/molecules/AlbumsRow.vue';
import ArtistsRow from '@/components/molecules/ArtistsRow.vue';
import TracksRow from '@/components/molecules/TracksRow.vue';
import { logFatal, logInfo } from '@/global/stores/debugger';
import { gqlClient } from '@/global/urql-client';
import { getRouteParam } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { SearchPageQuery } from '@/graphql/generated/graphql';
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';
import type { SimpleNavigableItem as SimpleNavigableItemClass } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem';
import { onMounted, ref } from 'vue';

const MAX_RESULTS_PER_CATEGORY = 50

async function onInput() {
  const searchTerms = query.value.trim()

  if (searchTerms.length === 0) {
    return
  }

  logInfo(`Performing search "${searchTerms}"...`)

  const start = Date.now()

  const { data, error } = await gqlClient.query(
    graphql(`
      query SearchPage($input: String!, $limit: Int!) {
        search(input: $input, limit: $limit) {
          tracks {
            ...AudioTrack
          }
          albums {
            ...Album
          }
          artists {
            ...Artist
          }
        }
      }
    `),
    {
      input: searchTerms,
      limit: MAX_RESULTS_PER_CATEGORY
    }
  )

  logInfo(`Performed search "${searchTerms}" in ${Date.now() - start} ms`)

  if (!data) {
    logFatal('Failed to perform search', error)
  }

  results.value = data.search
}

const query = ref(getRouteParam('query', ''))
const results = ref<SearchPageQuery['search'] | null>(null)
const inputRef = ref<HTMLInputElement | null>(null)
const navItem = ref<SimpleNavigableItemClass | null>(null)

onMounted(() => {
  if (!navItem.value) {
    logFatal('Nav item reference not initialized yet')
  }

  navItem.value.requestFocus()
})
</script>

<template>
  <div class="p-2.5 text-center">
    <SimpleNavigableItem @focus="inputRef?.focus()" @unfocus="inputRef?.blur()" v-slot="{ item }">
      <Run @run="navItem = item" />

      <input class="w-1/3 p-3 text-lg border-none rounded-lg outline-none" type="text" ref="inputRef" v-model="query"
        @input="onInput" @change="onInput" />
    </SimpleNavigableItem>
  </div>

  <div v-if="results">
    <h2>Tracks {{ results.tracks.length }}</h2>

    <TracksRow :tracks="results.tracks" />

    <h2>Albums {{ results.albums.length }}</h2>

    <AlbumsRow :albums="results.albums" />

    <h2>Artists {{ results.artists.length }}</h2>

    <ArtistsRow :artists="results.artists" />
  </div>
</template>