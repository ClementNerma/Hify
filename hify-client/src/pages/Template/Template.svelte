<script lang="ts">
  import './Template.css'

  import { useLocation, useNavigate } from 'svelte-navigator'

  import { setPlayingAudioProgressRelative, toggleAudioPlayback } from '../../stores/audio-player'

  import { ROUTES } from '../../routes'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigablePage from '../../navigable/NavigablePage/NavigablePage.svelte'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import TabNav from '../../molecules/TabNav/TabNav.svelte'
  import { handleInput } from '../../navigable/input-manager'
  import { playNextTrack, playPreviousTrackOrRewind } from '../../stores/play-queue'

  const navigate = useNavigate()
  const location = useLocation()

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
</script>

<NavigablePage>
  <NavigableWithHandlers onBack={() => navigate(-1)}>
    <NavigableList>
      <TabNav
        tabs={[
          { label: 'Home', uri: ROUTES.home },
          { label: 'Albums', uri: ROUTES.albums },
          { label: 'Search', uri: ROUTES.search },
          { label: 'Now Playing', uri: ROUTES.nowPlaying },
          { label: 'Dev Tools', uri: ROUTES.devTools },
        ]}
      />

      <slot />
    </NavigableList>
  </NavigableWithHandlers>
</NavigablePage>
