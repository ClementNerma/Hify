<script setup lang="ts">
import { getAlbumArtUrl } from '@/global/constants'
import { humanReadableDuration } from '@/global/stores/audio-player'
import { isApproachingListEnd } from '@/global/utils'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import NavigableList from '@/navigable/vue/components/NavigableList.vue'
import NavigableTrack from '../atoms/NavigableTrack.vue'
import TrackRating from '../atoms/TrackRating.vue'

defineProps<{
  tracks: AudioTrackFragment[]
  hasMore: boolean
  showArtistsName?: boolean
  feedMore: () => Promise<void>
}>()
</script>

<template>
  <NavigableList>
    <table class="mt-2.5 border-collapse whitespace-nowrap">
      <tbody>
        <tr
          v-for="track, i in tracks"
          :key="track.id"
          class="[&>td]:p-2.5"
          :class="i > 0 ? ['border-0 border-t border-solid border-gray-700'] : []"
        >
          <td>
            <img
              :width="50"
              :height="50"
              :src="getAlbumArtUrl(track.metadata.tags.album, 'medium')"
            />
          </td>
          <td class="w-full">
            <NavigableTrack
              :tracks
              :context="{ context: 'artist' }"
              :track
              @focus="isApproachingListEnd(i, tracks.length) && feedMore()"
            >
              <span>{{ track.metadata.tags.title }}</span>
            </NavigableTrack>
          </td>
          <td>
            <span v-if="track.computedRating">
              <TrackRating :rating="track.computedRating" />
            </span>
          </td>
          <td class="text-right">{{ humanReadableDuration(track.metadata.duration) }}</td>
          <td>
            {{ track.metadata.tags.album.name }}

            <span v-if="showArtistsName">
              ({{track.metadata.tags.album.albumArtists.map(artist => artist.name).join(', ')}})
            </span>
          </td>
        </tr>
      </tbody>
    </table>
  </NavigableList>
</template>
