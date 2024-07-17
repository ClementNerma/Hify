<script setup lang="ts">
import Button from '@/components/atoms/Button.vue';
import Checkbox from '@/components/atoms/Checkbox.vue';
import Emoji from '@/components/atoms/Emoji.vue';
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import NavigableTrack from '@/components/atoms/NavigableTrack.vue';
import TrackRating from '@/components/atoms/TrackRating.vue';
import { getAlbumArtUrl } from '@/global/constants';
import { humanReadableDuration } from '@/global/stores/audio-player';
import { showContextMenu } from '@/global/stores/context-menu';
import { enqueue, playNewQueueFromBeginning } from '@/global/stores/play-queue';
import { dedup, filterMap, getRouteParam, hasMinimumRating, isDefined, shuffle } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';
import NavigableList from '@/navigable/vue/components/NavigableList.vue';
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue';
import router from '@/router';
import { useQuery } from '@urql/vue';
import { computed, ref } from 'vue';

const albumId = getRouteParam('id')

function getAlbumInfos(filteredTracks: AudioTrackFragment[]) {
  const discs = dedup(filterMap(filteredTracks, (track) => track.metadata.tags.disc)).map((num) => ({
    number: num.toString(),
    tracks: filteredTracks.filter((track) => track.metadata.tags.disc === num),
  }))

  const tracksWithoutDisc = filteredTracks.filter((track) => !isDefined(track.metadata.tags.disc))

  if (tracksWithoutDisc.length > 0) {
    discs.unshift({ number: '?', tracks: tracksWithoutDisc })
  }

  return {
    totalDuration: filteredTracks.map((track) => track.metadata.duration).reduce((a, x) => a + x, 0),
    discs,
  }
}

const { data, fetching, error } = useQuery({
  query: graphql(`
    query AlbumPage($albumId: String!) {
      album(id: $albumId) {
        ...Album
        year
        genres {
          id
          name
        }
        tracks {
          ...AudioTrack
        }
      }
    }
  `),
  variables: { albumId }
})

const onlyShowGreatSongs = ref(false)

const album = computed(() => data.value?.album)
const filteredTracks = computed(() => album.value && onlyShowGreatSongs.value ? album.value.tracks.filter((track) => hasMinimumRating(track, 8)) : album.value?.tracks)
const infos = computed(() => filteredTracks.value && getAlbumInfos(filteredTracks.value))
</script>

<template>
  <LoadingIndicator v-if="fetching" :error="error?.message" />

  <div class="mt-2.5 ml-[15%] w-[70%]" v-if="album && filteredTracks && infos">
    <NavigableList>
      <div class="flex flex-row">
        <img class="art" :width="192" :height="192" :src="getAlbumArtUrl(album)" />

        <div class="flex flex-col mt-2.5 ml-2.5 gap-2.5 w-full">
          <div class="text-3xl">
            {{ album.name }}
          </div>

          <!-- TODO: implement attr "just-for-style" -->
          <NavigableItem v-if="album.year" just-for-style>
            🕒 {{ album.year }}
          </NavigableItem>

          <NavigableRow>
            <NavigableItem v-for="artist in album.albumArtists" :key="artist.id" display="inline"
              @press="router.push({ name: 'artist', params: { id: artist.id } })">
              🎤 {{ artist.name }}
            </NavigableItem>
          </NavigableRow>

          <NavigableRow>
            <NavigableItem v-for="genre in album.genres" :key="genre.id" display="inline"
              @press="router.push({ name: 'genre', params: { id: genre.id } })">
              🎵 {{ genre.name }}
            </NavigableItem>
          </NavigableRow>

          <div>
            ⌛ {{ humanReadableDuration(infos.totalDuration) }} /
            {{ filteredTracks.length }} track{{ filteredTracks.length > 1 ? 's' : '' }}

            <span v-if="infos.discs.length > 1">/ {{ infos.discs.length }} discs</span>
          </div>

          <NavigableRow>
            <Checkbox v-model="onlyShowGreatSongs">Only show great songs</Checkbox>

            <Button @press="enqueue(filteredTracks!, 'next')" @long-press="showContextMenu([
              {
                label: 'Queue at the end',
                onPress: () => enqueue(filteredTracks!, 'end'),
              }
            ])">
              <Emoji>▶️</Emoji> Play next
            </Button>

            <Button @press="() => {
              playNewQueueFromBeginning(shuffle(filteredTracks!), null)
              router.push({ name: 'now-playing' })
            }">
              <Emoji>🔀</Emoji> Shuffle
            </Button>
          </NavigableRow>
        </div>
      </div>
    </NavigableList>

    <div v-for="disc in infos.discs" :key="disc.number">
      <h2 v-if="infos.discs.length > 1">
        Disc {{ disc.number }}
      </h2>

      <table class="mt-2.5 w-full border-collapse">
        <tbody>
          <NavigableList>
            <NavigableTrack v-for="track, i in disc.tracks" :key="track.id" :tracks="filteredTracks"
              :context="{ context: 'album' }" :track>
              <tr class="w-full [&>td]:p-2.5" :class="i > 0 ? ['border-0 border-t border-solid border-gray-700'] : []">
                <td>{{ track.metadata.tags.trackNo }}</td>
                <td class="w-full">{{ track.metadata.tags.title }}</td>
                <td>
                  <span v-if="track.computedRating">
                    <TrackRating :rating="track.computedRating" />
                  </span>
                </td>
                <td class="text-right">{{ humanReadableDuration(track.metadata.duration) }}</td>
              </tr>
            </NavigableTrack>
          </NavigableList>
        </tbody>
      </table>
    </div>
  </div>
</template>
