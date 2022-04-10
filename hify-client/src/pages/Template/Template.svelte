<script lang="ts">
  import './Template.css'

  import { useLocation, useNavigate } from 'svelte-navigator'

  import { setPlayingAudioProgressRelative, toggleAudioPlayback } from '../../stores/audio-player'

  import { ROUTES } from '../../routes'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigablePage from '../../navigable/NavigablePage/NavigablePage.svelte'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import NavBar from '../../molecules/NavBar/NavBar.svelte'
  import { handleInput, registerLongPressableKeys } from '../../navigable/input-manager'
  import { playNextTrack, playPreviousTrackOrRewind } from '../../stores/play-queue'
  import { onMount } from 'svelte'

  const navigate = useNavigate()
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
      <NavBar
        tabs={[
          { label: 'Home', uri: ROUTES.home },
          { label: 'Albums', uri: ROUTES.albums },
          { label: 'Search', uri: ROUTES.search },
          { label: 'Now Playing', uri: ROUTES.nowPlaying },
          { label: 'Dev Tools', uri: ROUTES.devTools },
        ]}
        bind:requestFocus
      />

      <slot />
    </NavigableList>
  </NavigableWithHandlers>
</NavigablePage>
