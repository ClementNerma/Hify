<script lang="ts">
  import { getAlbumArtUri } from '../../rest-api'
  import { readableAudioPaused } from '../../stores/audio-player'
  import { currentTrack } from '../../stores/play-queue'

  import { setupDistractionFreeListener } from '../../stores/distraction-free'
  import DistractionFreeTogglable from '../../atoms/DistractionFreeTogglable/DistractionFreeTogglable.svelte'
  import { distractionFreeMode } from '../../stores/distraction-free'
  import { onMount, onDestroy } from 'svelte'
  import { blackBackground } from '../../stores/black-background'
  import NowPlayingBottomPanel from './NowPlayingBottomPanel.svelte'
  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'

  const ignoredKeys = ['MediaPlayPause', 'MediaRewind', 'MediaFastForward', 'Escape']

  const setDistractionFree = setupDistractionFreeListener(3000, ignoredKeys, () => $readableAudioPaused === false)

  function onKeyPress(key: string): boolean {
    const dfMode = $distractionFreeMode

    if (!dfMode && key === 'Escape') {
      setDistractionFree(true)
      return false
    }

    if (dfMode && !ignoredKeys.includes(key)) {
      setDistractionFree(false)
      return false
    }

    return true
  }

  onMount(() => blackBackground.set(true))
  onDestroy(() => blackBackground.set(false))

  currentTrack.subscribe(() => setDistractionFree(false))
</script>

{#if !$currentTrack}
  <h2 class="no-playing">Nothing currently playing or queue is loading</h2>
{:else}
  <img
    class="album-art"
    class:darkened={!$distractionFreeMode}
    width={$distractionFreeMode ? '' : 250}
    height={$distractionFreeMode ? '' : 250}
    src={getAlbumArtUri($currentTrack.metadata.tags.album.id)}
    alt=""
  />
{/if}

<DistractionFreeTogglable>
  <NavigableWithHandlers {onKeyPress}>
    <NowPlayingBottomPanel currentTrack={$currentTrack} />
  </NavigableWithHandlers>
</DistractionFreeTogglable>

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

    transition: opacity 0.5s;
  }

  .album-art.darkened {
    opacity: 0.5;
  }
</style>
