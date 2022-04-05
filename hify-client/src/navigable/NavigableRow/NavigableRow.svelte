<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, NavigableAttachedData, setChildrenNavigable } from '../navigation'
  import { NavigableRow } from './NavigableRow'

  export let attachedData: NavigableAttachedData = null

  const nav = getParentNavigable()
  const row = new NavigableRow(nav, attachedData)

  nav.append(row)

  setChildrenNavigable(row)

  onDestroy(() => nav.remove(row))
</script>

<slot />
