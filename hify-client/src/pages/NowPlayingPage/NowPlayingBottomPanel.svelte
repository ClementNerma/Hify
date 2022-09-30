<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { bind, formatDate } from '../../globals/utils'

  import {
    humanReadableDuration,
    readableAudioPaused,
    readableAudioProgress,
    setPlayingAudioProgress,
    toggleAudioPlayback,
  } from '../../stores/audio-player'

  import { AudioTrackFragment } from '../../graphql/generated'

  import SimpleNavigableItem from '../../navigable/headless/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import NavigableRow from '../../navigable/headless/NavigableRow/NavigableRow.svelte'
  import ProgressRange from '../../navigable/ui/atoms/ProgressRange/ProgressRange.svelte'
  import { ROUTES } from '../../routes'
  import QueueGallery from '../../organisms/QueueGallery/QueueGallery.svelte'
  import NavigableOne from '../../navigable/headless/NavigableOne/NavigableOne.svelte'
  import Column from '../../navigable/ui/molecules/Column/Column.svelte'

  export let currentTrack: AudioTrackFragment | false
  let isQueueFocused = false
</script>

<div class="player-bottom" class:isQueueFocused class:noCurrentTrack={!currentTrack}>
  <Column>
    <Column>
      {#if currentTrack}
        {@const tags = currentTrack.metadata.tags}
        {@const album = tags.album}
        {@const artists = tags.artists.length > 0 ? tags.artists : album.albumArtists}

        <div class="track-infos">
          <NavigableRow>
            <SimpleNavigableItem onPress={bind(tags, (tags) => void navigate(ROUTES.searchTerms(tags.title)))}>
              <div class="track-info">🎵 {tags.title}</div>
            </SimpleNavigableItem>
            <SimpleNavigableItem onPress={bind(album, (album) => void navigate(ROUTES.album(album.id)))}>
              <div class="track-info">💿 {album.name}</div>
            </SimpleNavigableItem>
            {#if tags.date}
              <SimpleNavigableItem justForStyle>
                <div class="track-info">🕒 {formatDate(tags.date)}</div>
              </SimpleNavigableItem>
            {/if}

            {#each artists as artist}
              <SimpleNavigableItem onPress={bind(artist.id, (id) => navigate(ROUTES.artist(id)))}>
                <div class="track-info">🎤 {artist.name}</div>
              </SimpleNavigableItem>
            {/each}
          </NavigableRow>
        </div>

        <div class="player-time">
          <div class="track-progress">
            {#if $readableAudioProgress !== null}
              {humanReadableDuration($readableAudioProgress)}
            {:else}
              --:--
            {/if}
            {#if $readableAudioPaused}
              ⏸️
            {/if}
          </div>
          <div class="track-duration">
            {humanReadableDuration(currentTrack.metadata.duration)}
          </div>
        </div>

        <div class="progress-range">
          <ProgressRange
            max={currentTrack.metadata.duration}
            value={$readableAudioProgress}
            onChange={setPlayingAudioProgress}
            onPress={toggleAudioPlayback}
            directionalAmount={30}
          />
        </div>
      {/if}
    </Column>

    <div class="play-queue-gallery">
      <NavigableOne
        onFocusChangeCallback={(focused) => {
          isQueueFocused = focused
        }}
      >
        <QueueGallery />
      </NavigableOne>
    </div>
  </Column>
</div>

<style>
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

  .track-infos {
    display: flex;
    flex-direction: row;
    font-size: 1.2rem;
  }

  .track-info {
    padding: 5px;
  }

  .progress-range,
  .progress-range :global(input) {
    width: calc(100% - 5px);
    color: red;
  }

  .player-time {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    padding: 0px 10px;
    transition: all 1s;
  }
</style>
