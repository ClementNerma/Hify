<script setup lang="ts">
import Button from '@/components/atoms/Button.vue';
import Centered from '@/components/atoms/Centered.vue';
import IndexUpdater from '@/components/atoms/IndexUpdater.vue';
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import MixButton from '@/components/atoms/MixButton.vue';
import AlbumsRow from '@/components/molecules/AlbumsRow.vue';
import TracksRow from '@/components/molecules/TracksRow.vue';
import { MIN_GREAT_RATING } from '@/global/constants';
import { graphql } from '@/graphql/generated';
import { MixOrdering, type HomePageQuery } from '@/graphql/generated/graphql';
import Row from '@/navigable/ui/molecules/Row/Row.vue';
import router from '@/router';
import { useQuery } from '@urql/vue';
import { computed, ref } from 'vue';

const { data, executeQuery } = useQuery({
  query: graphql(`
    query HomePage($randomItemsParams: FeedParams!) {
      generateFeed(input: $randomItemsParams) {
        lastListenedTo {
          ...AudioTrack
        }

        periodicallyPopularTracks {
          ...AudioTrack
        }

        mostRecentAlbums {
          ...Album
        }
      }
    }
  `),
  variables: {
    randomItemsParams: {
      maxItems: 100,
      minRating: 8,
    }
  }
})


const feed = computed(() => data.value?.generateFeed)
const statsBox = ref(false)
</script>

<template>
  <LoadingIndicator v-if="!feed" />

  <template v-else>
    <Centered>
      <h2>Welcome!</h2>

      <MixButton :mixParams="{
        source: { allTracks: '-' },
        ordering: MixOrdering.Random,
        minRating: MIN_GREAT_RATING,
      }" />
    </Centered>

    <Centered>
      <h3>Tracks you currently like to listen to:</h3>
    </Centered>

    <TracksRow :tracks="feed.periodicallyPopularTracks" />

    <Centered>
      <h3>Last songs you listened to:</h3>
    </Centered>

    <TracksRow :tracks="feed.lastListenedTo" />

    <Centered>
      <h3>Last albums to collection:</h3>
    </Centered>

    <AlbumsRow :albums="feed.mostRecentAlbums" />

    <Centered>
      <h3>Tools</h3>
    </Centered>

    <Row>
      <IndexUpdater @updated="executeQuery" />
      <Button @press="router.push({ name: 'devtools' })">👷 Devtools</Button>
      <Button @press="statsBox = !statsBox">Show me some stats</Button>
    </Row>
  </template>
</template>
