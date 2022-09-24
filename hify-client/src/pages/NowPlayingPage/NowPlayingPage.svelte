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
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import { readableDistractionFreeMode } from '../../stores/distraction-free'
  import { onMount, onDestroy } from 'svelte'
  import { blackBackground } from '../../stores/black-background'

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
    <NavigableRow>
      <NavigableList>
        <div class="track-infos">
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
      </NavigableList>

      <NavigableList>
        <div class="track-artists">
          <NavigableRow>
            {#each album.albumArtists as albumArtist}
              <SimpleNavigableItem onPress={bind(albumArtist.id, (id) => navigate(ROUTES.artist(id)))}>
                <span class="track-info">{albumArtist.name} 🎤</span>
              </SimpleNavigableItem>
            {/each}
          </NavigableRow>
        </div>
      </NavigableList>
    </NavigableRow>

    <div class="player-bottom">
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
    </div>

    <div class="play-queue-gallery">
      <QueueGallery />
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
    z-index: 2;
    position: fixed;
  }

  .album-art:not(.centered) {
    top: calc(35% - 250px / 2);
    left: calc(50% - 250px / 2);

    transition: all 0.5s linear;

    /* Just so the image doesn't get wider in the beginning of the transition
       when coming from .centered */
    -o-object-fit: contain;
    object-fit: contain;
  }

  .album-art.centered {
    top: 5%;
    left: 5%;

    width: 90%;
    height: 90%;

    margin: auto;
    overflow: auto;

    -o-object-fit: contain;
    object-fit: contain;

    transition: all 1s linear;
  }

  .track-infos {
    display: flex;
    flex-direction: column;
    position: fixed;
    top: calc(35% - 250px / 2 + 33px);
    width: calc(50% - 250px / 2 - 10px);
    left: 5%;
    font-size: 1.2rem;
  }

  .track-artists {
    display: flex;
    flex-direction: column;
    position: fixed;
    top: calc(35% - 250px / 2 + 33px);
    right: 5%;
    font-size: 1.2rem;
  }

  .track-info {
    display: inline-block;
    padding: 5px;
  }

  .player-bottom {
    position: fixed;
    bottom: 5%;

    top: calc(35% + 250px / 2 + 15px);
    left: 10%;
    right: 10%;
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

  .play-queue-gallery {
    position: fixed;
    left: 5%;
    right: 5%;
    bottom: 20px;
  }
</style>
