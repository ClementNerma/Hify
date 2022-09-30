<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, Props, setChildrenNavigable } from '../navigation'
  import { NavigableGrid } from './NavigableGrid'

  export let position: Props<NavigableGrid>['position'] = null
  export let hasFocusPriority: Props<NavigableGrid>['hasFocusPriority'] = null

  export let columns: Props<NavigableGrid>['columns']
  export let lazyLoader: Props<NavigableGrid>['lazyLoader'] = undefined

  const gridProps = (): Props<NavigableGrid> => ({
    position,
    hasFocusPriority,
    columns,
    lazyLoader,
  })

  const nav = getParentNavigable()
  const grid = new NavigableGrid(nav, gridProps())

  nav.append(grid)

  setChildrenNavigable(grid)

  afterUpdate(() => grid.updateProps(gridProps()))

  onDestroy(() => nav.remove(grid))

  export const requestFocus = () => grid.requestFocus()
</script>

<slot nav={grid} />
