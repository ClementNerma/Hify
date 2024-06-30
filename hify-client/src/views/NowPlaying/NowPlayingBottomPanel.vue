<script setup lang="ts">
import ProgressiveRow, { type ProgressiveRowExposeType } from '@/components/molecules/ProgressiveRow.vue';
import ModifiableTrackRating from '@/components/atoms/ModifiableTrackRating.vue';
import AudioProgressBar from '@/components/atoms/AudioProgressBar.vue';
import With from '@/components/atoms/With.vue';
import Card from '@/components/molecules/Card.vue';
import { ctxMenuOptions } from '@/global/context-menu-items';
import { humanReadableDuration, readableAudioPaused, readableAudioProgress, setPlayingAudioProgressRelative, toggleAudioPlayback } from '@/global/stores/audio-player';
import { enableOpacitor } from '@/global/stores/opacitor';
import { currentTrack, playTrackFromCurrentQueue, readablePlayQueue } from '@/global/stores/play-queue';
import { formatDate } from '@/global/utils';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import router from '@/router';
import { ref, watch } from 'vue';
import { getAlbumArtUrl } from '@/global/constants';
import NavigableList from '@/navigable/vue/components/NavigableList.vue';
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';
import { showContextMenu } from '@/global/stores/context-menu';

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
        onQueueEdition: () => queueGalleryRef.value?.requestFocus(position)
      },
    ),
  )
}

const queueGalleryRef = ref<ProgressiveRowExposeType | null>(null)

watch(() => [queueGalleryRef.value, readablePlayQueue.value.position] as const, ([gallery, position]) => {
  if (gallery !== null && position !== null) {
    gallery.jumpUnfocusedPosition(position)
  }
})
</script>

<template>
  <div class="player-bottom" :class="{ isQueueFocused, noCurrentTrack: !currentTrack }">
    <NavigableList>
      <NavigableList v-if="currentTrack">
        <With :data="{ currentTrack, tags: currentTrack.metadata.tags }" v-slot="{ data: { currentTrack, tags } }">
          <div class="buttons">
            <NavigableRow>
              <NavigableItem @press="router.push({ name: 'search', params: { query: tags.title } })">
                <div class="track-info">🎵 {{ tags.title }}</div>
              </NavigableItem>

              <NavigableItem @press="router.push({ name: 'album', params: { id: tags.album.id } })">
                <div class="track-info">💿 {{ tags.album.name }}</div>
              </NavigableItem>

              <NavigableItem v-if="tags.date" just-for-style>
                <div class="track-info">🕒 {{ formatDate(tags.date) }}</div>
              </NavigableItem>

              <NavigableItem v-for="artist in tags.artists" :key="artist.id"
                @press="router.push({ name: 'artist', params: { id: artist.id } })">
                <div class="track-info">🎤 {{ artist.name }}</div>
              </NavigableItem>

              <NavigableItem @press="enableOpacitor = !enableOpacitor">
                <div class="option-button">
                  <span v-if="enableOpacitor">🔲</span>
                  <span v-else>🔳</span>
                </div>
              </NavigableItem>

              <ModifiableTrackRating :track="currentTrack" />
            </NavigableRow>
          </div>

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
            <AudioProgressBar :max="currentTrack.metadata.duration" :value="readableAudioProgress ?? 0"
              @press="toggleAudioPlayback"
              @direction="dir => setPlayingAudioProgressRelative(dir === 'left' ? -30 : 30)" />
          </div>
        </With>
      </NavigableList>

      <div class="play-queue-gallery">
        <NavigableList>
          <ProgressiveRow ref="queueGalleryRef" :items="readablePlayQueue.tracks" idProp="idInQueue"
            :initialPosition="readablePlayQueue.position ?? 0" @item-press="(_, pos) => playTrackFromCurrentQueue(pos)"
            @item-long-press="showTrackCtxMenu" @focus-change="focused => { isQueueFocused = focused }"
            v-slot="{ item: track, position, focused }">
            <Card :title="track.metadata.tags.title" :box-size="80" :art-url="getAlbumArtUrl(track.metadata.tags.album)"
              :opacity="readablePlayQueue.position === position ? 1 : focused ? 0.7 : 0.2" />
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

.buttons {
  display: flex;
  flex-direction: row;
  align-items: end;
  font-size: 1.2rem;
}

.track-info,
.option-button {
  padding: 5px;
  align-self: stretch;
}

.progress-range,
.progress-range :global(input) {
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