<script setup lang="ts">
import { useQuery } from '@urql/vue'
import Button from '@/components/atoms/Button.vue'
import Centered from '@/components/atoms/Centered.vue'
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue'
import MixButton from '@/components/atoms/MixButton.vue'
import { getRouteParam } from '@/global/utils'
import { graphql } from '@/graphql/generated'
import { MixOrdering } from '@/graphql/generated/graphql'
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue'
import router from '@/router'
import ArtistAlbums from './Artist/ArtistAlbums.vue'
import ArtistTrackParticipations from './Artist/ArtistTrackParticipations.vue'

const artistId = getRouteParam('id')

const { data, fetching, error } = useQuery({
	query: graphql(`
    query ArtistPage($artistId: String!) {
      artist(id: $artistId) {
        name
      }
    }
  `),
	variables: {
		artistId,
	},
})
</script>

<template>
  <LoadingIndicator
    v-if="fetching"
    :error="error?.message"
  />

  <h2 v-else-if="data && !data.artist">Artist was not found</h2>

  <template v-else-if="data?.artist">
    <Centered>
      <h2>Artist: {{ data.artist.name }}</h2>

      <NavigableRow>
        <MixButton :mix-params="{
          source: { artists: [artistId] },
          ordering: MixOrdering.Random,
          minRating: 8,
        }" />

        <Button @press="router.push({ name: 'artist-tracks', params: { id: artistId } })">
          Show all tracks 🎵
        </Button>
      </NavigableRow>
    </Centered>

    <ArtistAlbums :artist-id />

    <ArtistTrackParticipations :artist-id />
  </template>
</template>