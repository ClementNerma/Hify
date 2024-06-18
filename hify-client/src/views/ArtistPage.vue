<script setup lang="ts">
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import MixButton from '@/components/atoms/MixButton.vue';
import { getRouteParam } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import { MixOrdering } from '@/graphql/generated/graphql';
import { useQuery } from '@urql/vue';
import ArtistAlbums from './Artist/ArtistAlbums.vue';
import ArtistTrackParticipations from './Artist/ArtistTrackParticipations.vue';
import Centered from '@/components/atoms/Centered.vue';

const artistId = getRouteParam('id')

const { data, fetching } = useQuery({
  query: graphql(`
    query ArtistPage($artistId: String!) {
      artist(id: $artistId) {
        name
      }
    }
  `),
  variables: {
    artistId
  }
})
</script>

<template>
  <LoadingIndicator v-if="fetching" />

  <h2 v-else-if="data && !data.artist">Artist was not found</h2>

  <template v-else-if="data?.artist">
    <Centered>
      <h2>Artist: {{ data.artist.name }}</h2>

      <MixButton :mix-params="{
        source: { allTracks: '-' },
        ordering: MixOrdering.Random,
        minRating: 8,
        fromArtists: [artistId],
      }" />
    </Centered>

    <ArtistAlbums :artist-id />

    <ArtistTrackParticipations :artist-id />
  </template>
</template>