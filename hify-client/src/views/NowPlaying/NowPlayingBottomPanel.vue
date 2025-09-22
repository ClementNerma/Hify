<script setup lang="ts">
import { ref, watch } from 'vue'
import AudioProgressBar from '@/components/atoms/AudioProgressBar.vue'
import ModifiableTrackRating from '@/components/atoms/ModifiableTrackRating.vue'
import With from '@/components/atoms/With.vue'
import Card from '@/components/molecules/Card.vue'
import OneLineList from '@/components/molecules/OneLineList.vue'
import ProgressiveRow, { type ProgressiveRowExposeType } from '@/components/molecules/ProgressiveRow.vue'
import { getAlbumArtUrl } from '@/global/constants'
import { ctxMenuOptions } from '@/global/ctx-menu-content'
import {
  humanReadableDuration,
  readableAudioPaused,
  readableAudioProgress,
  setPlayingAudioProgressRelative,
  toggleAudioPlayback,
} from '@/global/stores/audio-player'
import { showContextMenu } from '@/global/stores/context-menu'
import { enableOpacitor } from '@/global/stores/opacitor'
import { currentTrack, playTrackFromCurrentQueue, readablePlayQueue } from '@/global/stores/play-queue'
import { formatDate } from '@/global/utils'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'
import NavigableList from '@/navigable/vue/components/NavigableList.vue'
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue'
import router from '@/router'

const isQueueFocused = ref(false)

function showTrackCtxMenu(track: AudioTrackFragment, position: number) {
  showContextMenu(
    ctxMenuOptions.forTrack(
      track,
      { fromMixId: null },
      {
        context: 'queue',
        isCurrent: readablePlayQueue.value.position === position,
        position,
        totalTracks: readablePlayQueue.value.tracks.length,
        onQueueEdition: () => queueGalleryRef.value?.requestFocus(position),
      },
    ),
  )
}

function moveInTrack(dir: 'left' | 'right') {
  setPlayingAudioProgressRelative(dir === 'left' ? -30 : 30)
}

const queueGalleryRef = ref<ProgressiveRowExposeType | null>(null)

watch(
  () => [queueGalleryRef.value, readablePlayQueue.value.position] as const,
  ([gallery, position]) => {
    if (gallery !== null && position !== null) {
      gallery.jumpUnfocusedPosition(position)
    }
  },
)
</script>

<template>
  <div
    class="player-bottom"
    :class="{ isQueueFocused, noCurrentTrack: !currentTrack }"
  >
    <NavigableList>
      <NavigableList v-if="currentTrack">
        <With
          :data="currentTrack.metadata.tags"
          v-slot="{ data: tags }"
        >
          <NavigableRow class="items-center">
            <NavigableItem @press="router.push({ name: 'search', params: { query: tags.title } })">
              <div>🎵 {{ tags.title }}</div>
            </NavigableItem>

            <NavigableItem @press="router.push({ name: 'album', params: { id: tags.album.id } })">
              <div>💿 {{ tags.album.name }}</div>
            </NavigableItem>

            <!-- TODO: implement "just-for-style" -->
            <NavigableItem v-if="tags.date">
              <div>🕒 {{ formatDate(tags.date) }}</div>
            </NavigableItem>

            <OneLineList
              prefix="🎤"
              :items="tags.artists.map(artist => ({ id: artist.id, label: artist.name }))"
              @press="artistId => router.push({ name: 'artist', params: { id: artistId } })"
            />

            <ModifiableTrackRating :track="currentTrack" />

            <NavigableItem @press="enableOpacitor = !enableOpacitor">
              <div>
                {{ enableOpacitor ? '🔲' : '🔳' }}
              </div>
            </NavigableItem>
          </NavigableRow>

          <div class="player-time">
            <div class="track-progress">
              <span v-if="readableAudioProgress !== null">
                {{ humanReadableDuration(readableAudioProgress) }}
              </span>
              <span v-else>
                --:--
              </span>

              <span v-if="readableAudioPaused">⏸️</span>
            </div>

            <div class="track-duration">
              {{ humanReadableDuration(currentTrack.metadata.duration) }}
            </div>
          </div>

          <div class="progress-range">
            <AudioProgressBar
              class="w-full"
              :max="currentTrack.metadata.duration"
              :value="readableAudioProgress ?? 0"
              @press="toggleAudioPlayback"
              @direction="moveInTrack"
            />
          </div>
        </With>
      </NavigableList>

      <div class="play-queue-gallery">
        <NavigableList>
          <ProgressiveRow
            ref="queueGalleryRef"
            :items="readablePlayQueue.tracks"
            disable-scroll
            :initialPosition="readablePlayQueue.position ?? 0"
            @item-press="(_, pos) => playTrackFromCurrentQueue(pos)"
            @item-long-press="showTrackCtxMenu"
            @focus-change="focused => { isQueueFocused = focused }"
            v-slot="{ item: track, position, focused }"
          >
            <Card
              :title="track.metadata.tags.title"
              :box-size="80"
              :art-url="getAlbumArtUrl(track.metadata.tags.album, 'medium')"
              :opacity="readablePlayQueue.position === position ? 1 : focused ? 0.7 : 0.2"
            />
          </ProgressiveRow>
        </NavigableList>
      </div>
    </NavigableList>
  </div>
</template>

<style scoped>
.player-bottom {
  position: fixed;

  left: 0;
  right: 0;
  bottom: -100px;

  padding-left: 5%;
  padding-right: 5%;
  padding-bottom: 1%;

  background-image: linear-gradient(to bottom, rgba(255, 0, 0, 0), rgba(30, 30, 30, 1));

  transition: bottom 0.3s;
}

.player-bottom.isQueueFocused,
.player-bottom.noCurrentTrack {
  bottom: 0px;

  transition: bottom 0.3s;
}

.progress-range {
  width: calc(100% - 5px);
}

.player-time {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  padding: 0px 10px;
  transition: all 1s;
}
</style>