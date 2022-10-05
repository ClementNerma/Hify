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
  import { get, writable } from 'svelte/store'
  import {
    darken,
    resetBackgroundGradient,
    setRadialGradient,
    setUniColor,
  } from '../../molecules/GradientBackground/GradientBackground.svelte'
  import { AudioTrackFragment } from '../../graphql/generated'
  import Emoji from '../../navigable/ui/atoms/Emoji/Emoji.svelte'

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
      setUniColor({ r: 0, g: 0, b: 0 })
    }
  })

  onDestroy(() => resetBackgroundGradient())

  const newTrackDisplay = writable<{ timeout: number; track: AudioTrackFragment } | null>(null)

  currentTrack.subscribe((track) => {
    if (!track) {
      newTrackDisplay.update((data) => {
        if (data !== null) {
          clearTimeout(data.timeout)
        }

        return null
      })

      return
    }

    const color = track.metadata.tags.album.art?.dominantColor ?? { r: 0, g: 0, b: 0 }

    setRadialGradient({
      centerColor: color,
      exteriorColor: darken(color, 2),
    })

    if ($distractionFreeMode) {
      newTrackDisplay.update((data) => {
        if (data !== null) {
          clearTimeout(data.timeout)
        }

        return {
          track,
          timeout: setTimeout(() => {
            newTrackDisplay.set(null)
          }, NEW_TRACK_DISPLAY_TIMEOUT),
        }
      })
    }
  })

  const COVER_SIZE = 250
  const NEW_TRACK_DISPLAY_TIMEOUT = 2000
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

{#if $newTrackDisplay}
  {@const tags = $newTrackDisplay.track.metadata.tags}

  <div class="new-track">
    <div class="title"><Emoji>🎵</Emoji> {tags.title}</div>
    <div class="album"><Emoji>💿</Emoji> {tags.album.name}</div>
  </div>
{/if}

<style>
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
  .new-track > * {
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

    transition: opacity 0.5s;

    filter: drop-shadow(0 0 1rem rgb(77, 77, 77));
  }

  .album-art.darkened {
    opacity: 0.5;
  }
</style>
