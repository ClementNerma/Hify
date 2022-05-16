<script lang="ts">
  import './Template.css'

  import { onMount } from 'svelte'
  import { useLocation, navigate } from 'svelte-navigator'

  import { setPlayingAudioProgressRelative, toggleAudioPlayback } from '../../stores/audio-player'
  import { playNextTrack, playPreviousTrackOrRewind } from '../../stores/play-queue'
  import { distractionFreeMode } from '../../stores/distraction-free'

  import { ROUTES } from '../../routes'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigablePage from '../../navigable/NavigablePage/NavigablePage.svelte'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import NavBar from '../../molecules/NavBar/NavBar.svelte'
  import { handleInput, registerLongPressableKeys } from '../../navigable/input-manager'
  import TplContextMenu from './TplContextMenu.svelte'
  import DistractionFreeTogglable from '../../atoms/DistractionFreeTogglable/DistractionFreeTogglable.svelte'
  import DevToolsPage from '../DevToolsPage/DevToolsPage.svelte'

  const location = useLocation()

  registerLongPressableKeys('MediaPlayPause', 'MediaRewind', 'MediaFastForward')

  handleInput((key, long) => {
    switch (key) {
      case 'MediaPlayPause':
        if (!long) {
          toggleAudioPlayback()
        } else {
          navigate($location.pathname === ROUTES.nowPlaying ? ROUTES.search : ROUTES.nowPlaying)
        }

        break

      case 'MediaRewind':
        if (!long) {
          setPlayingAudioProgressRelative(-10)
        } else {
          playPreviousTrackOrRewind()
        }

        break

      case 'MediaFastForward':
        if (!long) {
          setPlayingAudioProgressRelative(+10)
        } else {
          playNextTrack()
        }

        break

      default:
        return
    }

    return false
  })

  let requestFocus: () => void

  onMount(() => location.subscribe(requestFocus))
</script>

<NavigablePage>
  <NavigableWithHandlers onBack={() => navigate(-1)}>
    <NavigableList>
      <TplContextMenu />

      <DistractionFreeTogglable>
        <NavigableWithHandlers onLongPress={() => navigate(ROUTES.devTools)}>
          <NavBar
            tabs={[
              { label: 'Home', uri: ROUTES.home },
              { label: 'Albums', uri: ROUTES.albums },
              { label: 'Artists', uri: ROUTES.artists },
              { label: 'Search', uri: ROUTES.search },
              { label: 'Now Playing', uri: ROUTES.nowPlaying },
            ]}
            bind:requestFocus
          />
        </NavigableWithHandlers>
      </DistractionFreeTogglable>

      <slot />
    </NavigableList>
  </NavigableWithHandlers>
</NavigablePage>
