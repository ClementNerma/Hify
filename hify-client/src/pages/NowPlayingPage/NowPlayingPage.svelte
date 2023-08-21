<script lang="ts">
  import { readableAudioPaused } from '../../stores/audio-player'
  import { currentTrack } from '../../stores/play-queue'

  import { setupDistractionFreeListener } from '../../stores/distraction-free'
  import DistractionFreeTogglable from '../../atoms/DistractionFreeTogglable/DistractionFreeTogglable.svelte'
  import { distractionFreeMode } from '../../stores/distraction-free'
  import NowPlayingBottomPanel from './NowPlayingBottomPanel.svelte'
  import NavigableWithHandlers from '../../navigable/headless/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import { KeyPressHandling } from '../../navigable/input-manager'
  import ImgLoader from '../../atoms/ImgLoader/ImgLoader.svelte'
  import { writable } from 'svelte/store'
  import { AudioTrackFragment } from '../../graphql/generated'
  import Emoji from '../../navigable/ui/atoms/Emoji/Emoji.svelte'
  import NowPlayingPageBackground from './NowPlayingPageBackground.svelte'

  const ignoredKeys = ['MediaPlayPause', 'MediaRewind', 'MediaFastForward', 'Escape']

  const setDistractionFree = setupDistractionFreeListener(
    3000,
    ignoredKeys,
    () => $readableAudioPaused === false
  )

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

  const NEW_TRACK_DISPLAY_TIMEOUT = 2000
</script>

<NowPlayingPageBackground track={$currentTrack || null} dim={!$distractionFreeMode} />

{#if !$currentTrack}
  <h2 class="no-playing">Nothing currently playing or queue is loading</h2>
{:else}
  <ImgLoader art={$currentTrack.metadata.tags.album.art} let:src>
    <img class="album-art" class:darkened={!$distractionFreeMode} {src} alt="" />
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

    transition: opacity 0.3s;

    filter: drop-shadow(0 0 1em rgb(55, 55, 55));
  }

  .album-art.darkened {
    opacity: 0.5;
  }
</style>
