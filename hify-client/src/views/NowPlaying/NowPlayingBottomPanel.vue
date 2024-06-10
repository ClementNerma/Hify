<script setup lang="ts">
import ModifiableTrackRating from '@/components/atoms/ModifiableTrackRating.vue';
import ProgressRange from '@/components/atoms/ProgressRange.vue';
import With from '@/components/atoms/With.vue';
import Card from '@/components/molecules/Card.vue';
import ProgressiveRow from '@/components/molecules/ProgressiveRow.vue';
import { ctxMenuOptions } from '@/global/context-menu-items';
import { humanReadableDuration, readableAudioPaused, readableAudioProgress, setPlayingAudioProgress, toggleAudioPlayback } from '@/global/stores/audio-player';
import { enableOpacitor } from '@/global/stores/opacitor';
import { currentTrack, playTrackFromCurrentQueue, readablePlayQueue, type QueuedTrack } from '@/global/stores/play-queue';
import { bind, formatDate } from '@/global/utils';
import type { AudioTrackFragment } from '@/graphql/generated/graphql';
import NavigableRow from '@/navigable/headless/NavigableRow/NavigableRow.vue';
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';
import Column from '@/navigable/ui/molecules/Column/Column.vue';
import { showContextMenu } from '@/navigable/ui/molecules/ContextMenu/ContextMenu';
import router from '@/router';
import { ref } from 'vue';

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
      },
    ),
  )
}
</script>

<template>
  <div class="player-bottom" :class="{ isQueueFocused, noCurrentTrack: !currentTrack }">
    <Column>
      <Column v-if="currentTrack">
        <With :data="{ currentTrack, tags: currentTrack.metadata.tags }" v-slot="{ data: { currentTrack, tags } }">
          <div class="buttons">
            <NavigableRow>
              <SimpleNavigableItem @press="router.push({ name: 'search', params: { query: tags.title } })">
                <div class="track-info">🎵 {{ tags.title }}</div>
              </SimpleNavigableItem>

              <SimpleNavigableItem @press="router.push({ name: 'album', params: { id: tags.album.id } })">
                <div class="track-info">🎵 {{ tags.title }}</div>
              </SimpleNavigableItem>

              <SimpleNavigableItem v-if="tags.date" just-for-style>
                <div class="track-info">🕒 {{ formatDate(tags.date) }}</div>
              </SimpleNavigableItem>

              <SimpleNavigableItem v-for="artist in tags.artists" :key="artist.id"
                @press="router.push({ name: 'artist', params: { id: artist.id } })">
                <div class="track-info">🎤 {{ artist.name }}</div>
              </SimpleNavigableItem>

              <SimpleNavigableItem @press="enableOpacitor = !enableOpacitor">
                <div class="option-button">
                  <span v-if="enableOpacitor">🔲</span>
                  <span v-else>🔳</span>
                </div>
              </SimpleNavigableItem>

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
            <ProgressRange :max="currentTrack.metadata.duration" :value="readableAudioProgress"
              @change="setPlayingAudioProgress" @press="toggleAudioPlayback" :directional-amount="30" />
          </div>
        </With>
      </Column>

      <div class="play-queue-gallery">
        <Column>
          <ProgressiveRow :items="readablePlayQueue.tracks" idProp="idInQueue"
            :initialPosition="readablePlayQueue.position ?? 0" @item-press="(_, pos) => playTrackFromCurrentQueue(pos)"
            @item-long-press="showTrackCtxMenu" @focus-change="focused => { isQueueFocused = focused }"
            v-slot="{ item: track, position, focused }">
            <Card :title="track.metadata.tags.title" :box-size="80" :art="track.metadata.tags.album.art"
              :opacity="readablePlayQueue.position === position ? 1 : focused ? 0.7 : 0.2" />
          </ProgressiveRow>
        </Column>
      </div>
    </Column>
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