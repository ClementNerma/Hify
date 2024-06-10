<script setup lang="ts">
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import ArtistCard from '@/components/molecules/ArtistCard.vue';
import { MIN_GREAT_RATING } from '@/global/constants';
import { logFatal } from '@/global/stores/debugger';
import { generateAndPlayMix } from '@/global/stores/play-queue';
import { gqlClient } from '@/global/urql-client';
import { graphql } from '@/graphql/generated';
import { MixOrdering, type ArtistFragment, type ArtistsPageQuery } from '@/graphql/generated/graphql';
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';
import { showContextMenu } from '@/navigable/ui/molecules/ContextMenu/ContextMenu';
import Grid from '@/navigable/ui/organisms/Grid.vue';
import router from '@/router';
import { useQuery } from '@urql/vue';
import { computed, onMounted, ref } from 'vue';

const GENRES_PER_LINE = 6

const { data, fetching } = await useQuery({
  query: graphql(`
    query GenresPage {
      genres {
        id
        name
        albumsCount
      }
    }
  `),
})

const genres = computed(() => data.value?.genres)
</script>

<template>
  <LoadingIndicator v-if="fetching" />

  <template v-else-if="genres">
    <h2>List of all genres ({{ genres.length }}) and number of albums:</h2>

    <Grid :columns="GENRES_PER_LINE">
      <SimpleNavigableItem v-for="genre in genres" :key="genre.id"
        @press="router.push({ name: 'genre', params: { id: genre.id } })" @long-press="showContextMenu([
          {
            label: 'Mix me some magic ✨',
            onPress: () => {
              generateAndPlayMix({
                source: { allTracks: '-' },
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
      </SimpleNavigableItem>
    </Grid>
  </template>
</template>