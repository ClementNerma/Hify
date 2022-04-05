<script lang="ts">
  import { useLocation, useNavigate } from 'svelte-navigator'
  import { ROUTES } from '../../routes'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigablePage from '../../navigable/NavigablePage/NavigablePage.svelte'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import TabNav from '../../molecules/TabNav/TabNav.svelte'
  import { logVerbose } from '../../stores/audio/debugger'
  import { setPlayingAudioProgressRelative, toggleAudioPlayback } from '../../stores/audio/store'

  const navigate = useNavigate()
  const location = useLocation()

  function onKeyDown(e: KeyboardEvent) {
    logVerbose(`Key down: "${e.key}" (ctrl: ${e.ctrlKey}, alt: ${e.altKey}, shift: ${e.altKey})`)

    if (e.ctrlKey || e.altKey || e.shiftKey) {
      return
    }

    switch (e.key) {
      case 'Tab':
        if ($location.pathname === ROUTES.nowPlaying) {
          navigate(-1)
        } else {
          navigate(ROUTES.nowPlaying)
        }

        break

      case 'MediaPlayPause':
        toggleAudioPlayback()
        break

      case 'MediaRewind':
        setPlayingAudioProgressRelative(-10)
        break

      case 'MediaFastForward':
        setPlayingAudioProgressRelative(+10)
        break

      default:
        return
    }

    e.preventDefault()
    return false
  }
</script>

<NavigablePage {onKeyDown}>
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

<style>
  :global(navigable-item-wrapper :first-child) {
    border: 3px solid transparent;
    cursor: pointer;
  }

  :global(navigable-item-wrapper.focused :first-child, navigable-item-wrapper:hover :first-child) {
    outline: none;
    background-color: pink;
    border: 3px solid pink;
    border-radius: 5px;
  }
</style>
