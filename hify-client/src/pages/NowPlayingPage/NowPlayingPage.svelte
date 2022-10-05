<script lang="ts">
  import { readableAudioPaused } from '../../stores/audio-player'
  import { currentTrack } from '../../stores/play-queue'

  import { setupDistractionFreeListener } from '../../stores/distraction-free'
  import DistractionFreeTogglable from '../../atoms/DistractionFreeTogglable/DistractionFreeTogglable.svelte'
  import { distractionFreeMode } from '../../stores/distraction-free'
  import { onMount, onDestroy } from 'svelte'
  import NowPlayingBottomPanel from './NowPlayingBottomPanel.svelte'
  import NavigableWithHandlers from '../../navigable/headless/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import { KeyPressHandling } from '../../navigable/input-manager'
  import ImgLoader from '../../atoms/ImgLoader/ImgLoader.svelte'
  import { get } from 'svelte/store'
  import {
    resetBackgroundGradient,
    setRadialGradient,
    setVerticalGradient,
  } from '../../molecules/GradientBackground/GradientBackground.svelte'

  const ignoredKeys = ['MediaPlayPause', 'MediaRewind', 'MediaFastForward', 'Escape']

  const setDistractionFree = setupDistractionFreeListener(3000, ignoredKeys, () => $readableAudioPaused === false)

  function onKeyPress(key: string): KeyPressHandling {
    const dfMode = $distractionFreeMode

    if (!dfMode && key === 'Escape') {
      setDistractionFree(true)
      return KeyPressHandling.Intercepted
    }

    if (dfMode && !ignoredKeys.includes(key)) {
      setDistractionFree(false)
      return KeyPressHandling.Intercepted
    }

    return KeyPressHandling.Propagate
  }

  onMount(() => {
    if (!get(currentTrack)) {
      setVerticalGradient({
        startColor: { r: 0, g: 0, b: 0 },
        endColor: { r: 0, g: 0, b: 0 },
        colorSep: 0,
      })
    }
  })

  onDestroy(() => resetBackgroundGradient())

  currentTrack.subscribe((track) => {
    if (track) {
      const color = track.metadata.tags.album.art?.dominantColor ?? { r: 0, g: 0, b: 0 }

      setRadialGradient({
        centerColor: color,
        exteriorColor: {
          r: Math.round(color.r / 2),
          g: Math.round(color.g / 2),
          b: Math.round(color.b / 2),
        },
        colorSep: 50,
      })
    }

    setDistractionFree(false)
  })

  const COVER_SIZE = 250
</script>

{#if !$currentTrack}
  <h2 class="no-playing">Nothing currently playing or queue is loading</h2>
{:else}
  <ImgLoader art={$currentTrack.metadata.tags.album.art} let:src>
    <img
      class="album-art"
      class:darkened={!$distractionFreeMode}
      width={$distractionFreeMode ? '' : COVER_SIZE}
      height={$distractionFreeMode ? '' : COVER_SIZE}
      {src}
      alt=""
    />
  </ImgLoader>
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
