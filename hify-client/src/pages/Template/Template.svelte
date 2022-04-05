<script lang="ts">
  import { useLocation, useNavigate } from 'svelte-navigator'
  import { ROUTES } from '../../routes'

  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigablePage from '../../navigable/NavigablePage/NavigablePage.svelte'

  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import TabNav from '../../molecules/TabNav/TabNav.svelte'
  import { log } from '../../stores/audio/debugger'

  const navigate = useNavigate()
  const location = useLocation()

  function onKeyDown(e: KeyboardEvent) {
    log(`Key down: "${e.key}" (ctrl: ${e.ctrlKey}, alt: ${e.altKey}, shift: ${e.altKey})`)

    if (e.ctrlKey || e.altKey || e.shiftKey) {
      return
    }

    if (e.key === 'Tab') {
      if ($location.pathname === ROUTES.nowPlaying) {
        navigate(-1)
      } else {
        navigate(ROUTES.nowPlaying)
      }

      e.preventDefault()
      return false
    }

    return
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
    background-color: pink;
    border: 3px solid pink;
    border-radius: 5px;
  }
</style>
