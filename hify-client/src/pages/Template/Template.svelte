<script lang="ts">
  import '../../navigable/navigation.css'

  import { useLocation, navigate } from 'svelte-navigator'

  import { setPlayingAudioProgressRelative, toggleAudioPlayback } from '../../stores/audio-player'
  import { playNextTrack, playPreviousTrackOrRewind } from '../../stores/play-queue'

  import { ROUTES } from '../../routes'

  import NavigableList from '../../navigable/headless/NavigableList/NavigableList.svelte'
  import NavigablePage from '../../navigable/headless/NavigablePage/NavigablePage.svelte'

  import NavigableWithHandlers from '../../navigable/headless/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import NavBar from '../../molecules/NavBar/NavBar.svelte'
  import {
    handleInput,
    KeyPressHandling,
    registerLongPressableKeys,
  } from '../../navigable/input-manager'
  import DistractionFreeTogglable from '../../atoms/DistractionFreeTogglable/DistractionFreeTogglable.svelte'
  import ContextMenu from '../../navigable/ui/molecules/ContextMenu/ContextMenu.svelte'

  const location = useLocation()

  registerLongPressableKeys('MediaPlayPause', 'MediaRewind', 'MediaFastForward', 'Escape')

  handleInput((key, long) => {
    switch (key) {
      case 'MediaPlayPause':
      case 'p':
        if (!long) {
          toggleAudioPlayback()
        } else {
          navigate($location.pathname === ROUTES.nowPlaying ? ROUTES.search : ROUTES.nowPlaying)
        }

        break

      case 'MediaRewind':
      case 'r':
        if (!long) {
          setPlayingAudioProgressRelative(-10)
        } else {
          playPreviousTrackOrRewind()
        }

        break

      case 'MediaFastForward':
      case 'f':
        if (!long) {
          setPlayingAudioProgressRelative(+10)
        } else {
          playNextTrack()
        }

        break

      default:
        return
    }

    return KeyPressHandling.Propagate
  })
</script>

<div class="background" />

<NavigablePage>
  <NavigableWithHandlers onBack={() => navigate(-1)} onLongBack={() => window.location.reload()}>
    <NavigableList>
      <ContextMenu />

      <DistractionFreeTogglable>
        <NavigableWithHandlers onLongPress={() => navigate(ROUTES.devTools)}>
          <NavBar
            tabs={[
              { label: 'Home', uri: ROUTES.home },
              { label: 'History', uri: ROUTES.history },
              { label: 'Now Playing', uri: ROUTES.nowPlaying },
              { label: 'Albums', uri: ROUTES.albums },
              { label: 'Artists', uri: ROUTES.artists },
              { label: 'Genres', uri: ROUTES.genres },
              { label: 'Search', uri: ROUTES.search },
            ]}
          />
        </NavigableWithHandlers>
      </DistractionFreeTogglable>

      <slot />
    </NavigableList>
  </NavigableWithHandlers>
</NavigablePage>

<style>
  :global(html) {
    height: 100%;
    font-size: 12px;
  }

  :global(body) {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    color: rgb(230, 230, 230);
    overflow: auto;
  }

  .background {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: -3;

    background: linear-gradient(to bottom, #363636 0%, #080808 33%);
  }
</style>
