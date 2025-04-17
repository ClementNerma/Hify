<script setup lang="ts">
import { readableAudioPaused } from '@/global/stores/audio-player'
import { distractionFreeMode, setupDistractionFreeListener } from '@/global/stores/distraction-free'
import { currentTrack } from '@/global/stores/play-queue'
import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import { ref, watch } from 'vue'
import NowPlayingBackground from './NowPlaying/NowPlayingBackground.vue'
import DistractionFreeTogglable from '@/components/atoms/DistractionFreeTogglable.vue'
import NowPlayingBottomPanel from './NowPlaying/NowPlayingBottomPanel.vue'
import NowPlayingOpacitor from './NowPlaying/NowPlayingOpacitor.vue'
import Emoji from '@/components/atoms/Emoji.vue'
import { getAlbumArtUrl } from '@/global/constants'
import NavigableList from '@/navigable/vue/components/NavigableList.vue'
import { NavigationDirection } from '@/navigable'

const NEW_TRACK_DISPLAY_TIMEOUT = 2000

const setDistractionFree = setupDistractionFreeListener({
  delayMillis: 3000,
  darkeningCondition: () => readableAudioPaused.value === false,
  dontWakeUpForKeys: [
    { key: 'MediaPlayPause' },
    { key: 'MediaRewind' },
    { key: 'MediaFastForward' },
    { key: 'Escape' },
    { key: 'F4' },
    { key: 'ArrowLeft', shiftKey: true },
    { key: 'ArrowRight', shiftKey: true },
    { key: 'ArrowUp', shiftKey: true },
    { key: 'ArrowDown', shiftKey: true }
  ],
})

function interceptKeyPress(dir: NavigationDirection | null): boolean {
  const dfMode = distractionFreeMode.value

  if (!dfMode && dir === NavigationDirection.Back) {
    setDistractionFree(true)
    return true
  }

  return false
}

const newTrackDisplay = ref<{ timeout: number; track: AudioTrackFragment } | null>(null)

// TODO: improve this mess
watch(currentTrack, (track) => {
  if (!track) {
    if (newTrackDisplay.value !== null) {
      clearTimeout(newTrackDisplay.value.timeout)
    }

    newTrackDisplay.value = null
  } else if (distractionFreeMode.value) {
    if (newTrackDisplay.value !== null) {
      clearTimeout(newTrackDisplay.value.timeout)
    }

    newTrackDisplay.value = {
      track,
      timeout: setTimeout(() => {
        newTrackDisplay.value = null
      }, NEW_TRACK_DISPLAY_TIMEOUT),
    }
  }
})
</script>

<template>
  <NowPlayingBackground :track="currentTrack ?? null" :dim="!distractionFreeMode" />

  <h2 v-if="!currentTrack" class="no-playing">Nothing currently</h2>

  <img v-else class="album-art" :class="{ darkened: !distractionFreeMode }"
    :src="getAlbumArtUrl(currentTrack.metadata.tags.album)" />

  <DistractionFreeTogglable>
    <NavigableList :intercept-key-press>
      <NowPlayingBottomPanel />
    </NavigableList>
  </DistractionFreeTogglable>

  <NowPlayingOpacitor :visible="distractionFreeMode" />

  <div class="new-track" v-if="newTrackDisplay">
    <div class="title">
      <Emoji>🎵</Emoji> {{ newTrackDisplay.track.metadata.tags.title }}
    </div>
    <div class="album">
      <Emoji>💿</Emoji> {{ newTrackDisplay.track.metadata.tags.album.name }}
    </div>
  </div>
</template>

<style scoped>
.no-playing {
  position: fixed;
  top: 25%;
  width: 100%;
  text-align: center;
  font-size: 2rem;
}

.new-track {
  position: fixed;

  top: 10px;
  left: 10px;

  max-width: 300px;

  padding: 5px;

  border-radius: 5px;

  background-color: rgb(77, 77, 77);
  color: rgb(230, 230, 230);
}

/* TODO: remove experimental stuff */
.new-track>* {
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  line-clamp: 1;
  -webkit-box-orient: vertical;
}

.album-art {
  position: fixed;

  top: 10%;
  left: 10%;

  width: 80%;
  height: 80%;

  margin: auto;
  overflow: auto;

  -o-object-fit: contain;
  object-fit: contain;

  transition: opacity 0.3s;

  filter: drop-shadow(0 0 1em rgb(55, 55, 55));
}

.album-art.darkened {
  opacity: 0.5;
}
</style>
