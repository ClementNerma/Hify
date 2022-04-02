<script lang="ts">
  import { getContext, setContext } from 'svelte'
  import { NAVIGATION_CTX, usePageNavigator } from '../handlers/nav'

  const parentNav = getContext(NAVIGATION_CTX)

  if (parentNav) {
    throw new Error('Navigable view cannot have a parent navigator')
  }

  const nav = usePageNavigator(() => console.log('Going back!'))

  setContext(NAVIGATION_CTX, nav)
</script>

<slot />
