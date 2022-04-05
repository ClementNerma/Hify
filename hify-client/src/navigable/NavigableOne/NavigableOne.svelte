<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, NavigableAttachedData, setChildrenNavigable } from '../navigation'
  import { NavigableOne } from './NavigableOne'

  export let data: NavigableAttachedData = null

  const nav = getParentNavigable()
  const row = new NavigableOne(nav, data)

  nav.append(row)

  setChildrenNavigable(row)

  onDestroy(() => nav.remove(row))
</script>

<slot />
