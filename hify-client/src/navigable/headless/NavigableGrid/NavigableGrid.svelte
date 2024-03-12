<script lang="ts">
import { afterUpdate } from 'svelte'
import InternalNavWrapper from '../InternalNavWrapper.svelte'

import { getParentNavigable, Props, setChildrenNavigable } from '../../navigation'
import { NavigableGrid } from './NavigableGrid'

export let hasFocusPriority: Props<NavigableGrid>['hasFocusPriority'] = null

export let columns: Props<NavigableGrid>['columns']
export let lazyLoader: Props<NavigableGrid>['lazyLoader'] = undefined
export let distanceBeforeLazyLoading: Props<NavigableGrid>['distanceBeforeLazyLoading'] = undefined

const gridProps = (): Props<NavigableGrid> => ({
	hasFocusPriority,
	columns,
	lazyLoader,
	distanceBeforeLazyLoading,
})

const grid = new NavigableGrid(getParentNavigable(), gridProps())

setChildrenNavigable(grid)

afterUpdate(() => grid.updateProps(gridProps()))

export const requestFocus = () => grid.requestFocus()
</script>

<InternalNavWrapper navId={grid.id}>
  <slot nav={grid} />
</InternalNavWrapper>
