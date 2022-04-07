<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableGrid, NavigableGridProps } from './NavigableGrid'

  export let position: NavigableGridProps['position'] = null
  export let hasFocusPriority: NavigableGridProps['hasFocusPriority'] = null

  export let columns: NavigableGridProps['columns']
  export let lazyLoader: NavigableGridProps['lazyLoader'] = undefined

  const nav = getParentNavigable()
  const row = new NavigableGrid(nav, { position, hasFocusPriority, columns, lazyLoader })

  nav.append(row)

  setChildrenNavigable(row)

  afterUpdate(() => {
    row.position = position
    row.hasFocusPriority = hasFocusPriority
  })

  onDestroy(() => nav.remove(row))
</script>

<slot nav={row} />
