<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableGrid, NavigableGridProps } from './NavigableGrid'

  export let position: number | null = null

  export let columns: NavigableGridProps['columns']
  export let lazyLoader: NavigableGridProps['lazyLoader'] = undefined

  const nav = getParentNavigable()
  const row = new NavigableGrid(nav, position, { columns, lazyLoader })

  nav.append(row)

  setChildrenNavigable(row)

  onDestroy(() => nav.remove(row))
</script>

<slot nav={row} />
