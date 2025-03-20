<script setup lang="ts">
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue'
import { MIN_GREAT_RATING } from '@/global/constants'
import { showContextMenu } from '@/global/stores/context-menu'
import { generateAndPlayMix } from '@/global/stores/play-queue'
import { graphql } from '@/graphql/generated'
import { MixOrdering } from '@/graphql/generated/graphql'
import NavigableGrid from '@/navigable/vue/components/NavigableGrid.vue'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'
import router from '@/router'
import { useQuery } from '@urql/vue'
import { computed } from 'vue'

const GENRES_PER_LINE = 6

const { data, fetching, error } = useQuery({
	query: graphql(`
    query GenresPage {
      genres {
        id
        name
        albumsCount
      }
    }
  `),
	variables: {},
})

const genres = computed(() => data.value?.genres)
</script>

<template>
  <LoadingIndicator v-if="fetching" :error="error?.message" />

  <template v-else-if="genres">
    <h2>List of all genres ({{ genres.length }}) and number of albums:</h2>

    <NavigableGrid :columns="GENRES_PER_LINE">
      <NavigableItem v-for="genre in genres" :key="genre.id"
        @press="router.push({ name: 'genre', params: { id: genre.id } })" @long-press="showContextMenu([
          {
            label: 'Mix me some magic ✨',
            onPress: () => {
              generateAndPlayMix({
                source: { allTracks: true },
                ordering: MixOrdering.Random,
                minRating: MIN_GREAT_RATING,
                fromGenres: [genre.id],
              })
            }
          },
        ])">
        <div>
          <p>{{ genre.name }} ({{ genre.albumsCount }})</p>
        </div>
      </NavigableItem>
    </NavigableGrid>
  </template>
</template>