<script lang="ts">
import { afterUpdate } from 'svelte'

import { getParentNavigable, Props, setChildrenNavigable } from '../../navigation'
import InternalNavWrapper from '../InternalNavWrapper.svelte'
import { NavigableList } from './NavigableList'

export let hasFocusPriority: Props<NavigableList>['hasFocusPriority'] = null
export let trapped: Props<NavigableList>['trapped'] = undefined
export let lazyLoader: Props<NavigableList>['lazyLoader'] = undefined
export let distanceBeforeLazyLoading: Props<NavigableList>['distanceBeforeLazyLoading'] = undefined

const list = new NavigableList(getParentNavigable(), {
	hasFocusPriority,
	trapped,
	lazyLoader,
	distanceBeforeLazyLoading,
})

setChildrenNavigable(list)

afterUpdate(() => list.updateProps({ hasFocusPriority, trapped, lazyLoader, distanceBeforeLazyLoading }))
</script>

<InternalNavWrapper navId={list.id}>
  <slot nav={list} requestFocus={() => list.requestFocus()} />
</InternalNavWrapper>
