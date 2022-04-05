<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, NavigableAttachedData, setChildrenNavigable } from '../navigation'
  import { NavigableGrid, NavigableGridProps } from './NavigableGrid'

  export let columns: NavigableGridProps['columns']
  export let lazyLoader: NavigableGridProps['lazyLoader'] = undefined
  export let attachedData: NavigableAttachedData = null

  const nav = getParentNavigable()
  const row = new NavigableGrid(nav, { columns, lazyLoader, attachedData: attachedData })

  nav.append(row)

  setChildrenNavigable(row)

  onDestroy(() => nav.remove(row))
</script>

<slot />
