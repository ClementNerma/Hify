<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, NavigableAttachedData, setChildrenNavigable } from '../navigation'
  import { NavigableList } from './NavigableList'

  export let data: NavigableAttachedData = null

  const nav = getParentNavigable()
  const list = new NavigableList(nav, data)

  nav.append(list)

  setChildrenNavigable(list)

  onDestroy(() => nav.remove(list))
</script>

<slot />
