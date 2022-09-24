<script lang="ts">
  import { getAlbumArtUri } from '../../rest-api'
  import {
    humanReadableDuration,
    readableAudioPaused,
    readableAudioProgress,
    setPlayingAudioProgress,
    toggleAudioPlayback,
  } from '../../stores/audio-player'
  import { currentTrack } from '../../stores/play-queue'

  import { navigate } from 'svelte-navigator'
  import { bind, formatDate } from '../../utils'

  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import ProgressRange from '../../atoms/ProgressRange/ProgressRange.svelte'
  import { ROUTES } from '../../routes'
  import QueueGallery from '../../organisms/QueueGallery/QueueGallery.svelte'
  import { setupDistractionFreeListener } from '../../stores/distraction-free'
  import DistractionFreeTogglable from '../../atoms/DistractionFreeTogglable/DistractionFreeTogglable.svelte'
  import { readableDistractionFreeMode } from '../../stores/distraction-free'
  import { onMount, onDestroy } from 'svelte'
  import { blackBackground } from '../../stores/black-background'
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'

  $: tags = $currentTrack && $currentTrack.metadata.tags
  $: album = $currentTrack && $currentTrack.metadata.tags.album

  setupDistractionFreeListener(
    3000,
    ['MediaPlayPause', 'MediaRewind', 'MediaFastForward'],
    () => $readableAudioPaused === false,
  )

  onMount(() => blackBackground.set(true))
  onDestroy(() => blackBackground.set(false))
</script>

{#if !$currentTrack || !tags || !album}
  <h2 class="no-playing">Nothing currently playing or queue is loading</h2>
{:else}
  <img
    class="album-art {$readableDistractionFreeMode ? 'centered' : ''}"
    width={$readableDistractionFreeMode ? '' : 250}
    height={$readableDistractionFreeMode ? '' : 250}
    src={getAlbumArtUri(album.id)}
    alt=""
  />

  <DistractionFreeTogglable>
    <div class="player-bottom">
      <div class="track-infos">
        <NavigableRow>
          <div class="track-infos-row">
            <SimpleNavigableItem onPress={bind(tags, (tags) => void navigate(ROUTES.searchTerms(tags.title)))}>
              <div class="track-info">🎵 {tags.title}</div>
            </SimpleNavigableItem>
            <SimpleNavigableItem onPress={bind(album, (album) => void navigate(ROUTES.album(album.id)))}>
              <div class="track-info">💿 {album.name}</div>
            </SimpleNavigableItem>
            {#if tags.date}
              <div data-item-like-style>
                <div class="track-info">🕒 {formatDate(tags.date)}</div>
              </div>
            {/if}
          </div>
        </NavigableRow>

        <NavigableRow>
          <div class="track-infos-row">
            {#each album.albumArtists as albumArtist}
              <SimpleNavigableItem onPress={bind(albumArtist.id, (id) => navigate(ROUTES.artist(id)))}>
                <span class="track-info">🎤 {albumArtist.name}</span>
              </SimpleNavigableItem>
            {/each}
          </div>
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
          {humanReadableDuration($currentTrack.metadata.duration)}
        </div>
      </div>
      <div class="progress-range">
        <ProgressRange
          max={$currentTrack.metadata.duration}
          value={$readableAudioProgress}
          onChange={setPlayingAudioProgress}
          onPress={toggleAudioPlayback}
          directionalAmount={30}
        />
      </div>

      <div class="play-queue-gallery">
        <QueueGallery />
      </div>
    </div>
  </DistractionFreeTogglable>
{/if}

<style>
  .no-playing {
    position: fixed;
    top: 25%;
    width: 100%;
    text-align: center;
    font-size: 2rem;
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
  }

  .track-infos {
    display: flex;
    flex-direction: column;
    font-size: 1.2rem;
  }

  .track-infos-row {
    display: flex;
    flex-direction: row;
  }

  .track-info {
    display: inline-block;
    padding: 5px;
  }

  .player-bottom {
    position: fixed;

    left: 0;
    right: 0;
    bottom: 0;

    padding-left: 5%;
    padding-right: 5%;
    padding-bottom: 1%;

    background-image: linear-gradient(to bottom, rgba(255, 0, 0, 0), rgba(30, 30, 30, 1));
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
  }
</style>
