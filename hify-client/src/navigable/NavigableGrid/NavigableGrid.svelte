<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableGrid, NavigableGridProps } from './NavigableGrid'

  export let position: NavigableGridProps['position'] = null
  export let hasFocusPriority: NavigableGridProps['hasFocusPriority'] = null

  export let columns: NavigableGridProps['columns']
  export let lazyLoader: NavigableGridProps['lazyLoader'] = undefined

  const gridProps = (): NavigableGridProps => ({
    position,
    hasFocusPriority,
    columns,
    lazyLoader,
  })

  const nav = getParentNavigable()
  const grid = new NavigableGrid(nav, gridProps())

  nav.append(grid)

  setChildrenNavigable(grid)

  afterUpdate(() => {
    grid.position = position
    grid.hasFocusPriority = hasFocusPriority
    grid.props = gridProps()
  })

  onDestroy(() => nav.remove(grid))
</script>

<slot nav={grid} />
