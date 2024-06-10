<script setup lang="ts">
import Button from '@/components/atoms/Button.vue';
import Checkbox from '@/components/atoms/Checkbox.vue';
import Emoji from '@/components/atoms/Emoji.vue';
import ImgLoader from '@/components/atoms/ImgLoader.vue';
import LoadingIndicator from '@/components/atoms/LoadingIndicator.vue';
import NavigableTrack from '@/components/atoms/NavigableTrack.vue';
import TrackRating from '@/components/atoms/TrackRating.vue';
import { humanReadableDuration } from '@/global/stores/audio-player';
import { enqueue, playNewQueueFromBeginning } from '@/global/stores/play-queue';
import { dedup, filterMap, getRouteParam, hasMinimumRating, isDefined, shuffle } from '@/global/utils';
import { graphql } from '@/graphql/generated';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import NavigableList from '@/navigable/headless/NavigableList/NavigableList.vue';
import NavigableRow from '@/navigable/headless/NavigableRow/NavigableRow.vue';
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';
import { showContextMenu } from '@/navigable/ui/molecules/ContextMenu/ContextMenu';
import Row from '@/navigable/ui/molecules/Row/Row.vue';
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

const { data } = useQuery({
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
  <LoadingIndicator v-if="!data" />

  <div class="container" v-if="album && filteredTracks && infos">
    <NavigableList>
      <div class="header">
        <ImgLoader :art="album.art" v-slot="{ src }">
          <img class="art" :width="192" :height="192" :src />
        </ImgLoader>

        <div class="infos">
          <div class="name">
            {{ album.name }}
          </div>

          <SimpleNavigableItem v-if="album.year" just-for-style>
            🕒 {{ album.year }}
          </SimpleNavigableItem>

          <div class="artists">
            <NavigableRow>
              <SimpleNavigableItem v-for="artist in album.albumArtists" :key="artist.id"
                @press="router.push({ name: 'artist', params: { id: artist.id } })">
                <span class="artist">🎤 {{ artist.name }}</span>
              </SimpleNavigableItem>
            </NavigableRow>
          </div>

          <div class="genres">
            <NavigableRow>
              <SimpleNavigableItem v-for="genre in album.genres" :key="genre.id"
                @press="router.push({ name: 'genre', params: { id: genre.id } })">
                <span class="genre">🎤 {{ genre.name }}</span>
              </SimpleNavigableItem>
            </NavigableRow>
          </div>

          <SimpleNavigableItem just-for-style>
            <div class="length">
              ⌛ {{ humanReadableDuration(infos.totalDuration) }} /
              {{ filteredTracks.length }} track{{ filteredTracks.length > 1 ? 's' : '' }}

              <span v-if="infos.discs.length > 1">/ {{ infos.discs.length }} discs</span>
            </div>
          </SimpleNavigableItem>

          <Row>
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
          </Row>
        </div>
      </div>
    </NavigableList>

    <div v-for="disc in infos.discs" :key="disc.number">
      <h2 v-if="infos.discs.length > 1">
        Disc {{ disc.number }}
      </h2>

      <table>
        <tbody>
          <NavigableList>
            <NavigableTrack v-for="track, i in disc.tracks" :key="track.id" :tracks="filteredTracks"
              :context="{ context: 'album' }" :track>
              <tr :class="{ notFirst: i !== 0 }">
                <td class="trackno">{{ track.metadata.tags.trackNo }}</td>
                <td class="title">{{ track.metadata.tags.title }}</td>
                <td class="rating">
                  <span v-if="track.computedRating">
                    <TrackRating :rating="track.computedRating" />
                  </span>
                </td>
                <td class="duration">{{ humanReadableDuration(track.metadata.duration) }}</td>
              </tr>
            </NavigableTrack>
          </NavigableList>
        </tbody>
      </table>
    </div>
  </div>
</template>


<style scoped>
.container {
  margin-top: 10px;
  margin-left: 15%;
  width: 70%;
}

.header {
  display: flex;
  flex-direction: row;
}

.infos {
  display: flex;
  flex-direction: column;
  margin-top: 10px;
  margin-left: 10px;
  gap: 10px;
}

.infos .name {
  font-size: 2rem;
  font-weight: bold;
}

table {
  margin-top: 10px;
  width: 100%;
  border-collapse: collapse;
}

tr {
  width: 100%;
}

tr.notFirst {
  border-top: 1px solid rgb(50, 50, 50);
}

td {
  padding: 10px;
}

td.title {
  width: 100%;
}

td.duration {
  text-align: right;
}
</style>
