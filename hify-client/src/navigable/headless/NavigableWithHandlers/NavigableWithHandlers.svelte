<script lang="ts">
import { afterUpdate } from 'svelte'

import { getParentNavigable, Props, setChildrenNavigable } from '../../navigation'
import InternalNavWrapper from '../InternalNavWrapper.svelte'
import { NavigableWithHandlers } from './NavigableWithHandlers'

export let hasFocusPriority: Props<NavigableWithHandlers>['hasFocusPriority'] = null

export let onPress: Props<NavigableWithHandlers>['onPress'] = undefined
export let onLongPress: Props<NavigableWithHandlers>['onLongPress'] = undefined
export let onBack: Props<NavigableWithHandlers>['onBack'] = undefined
export let onLongBack: Props<NavigableWithHandlers>['onLongBack'] = undefined
export let onKeyPress: Props<NavigableWithHandlers>['onKeyPress'] = undefined

const containerProps = (): Props<NavigableWithHandlers> => ({
	hasFocusPriority,
	onPress,
	onLongPress,
	onBack,
	onLongBack,
	onKeyPress,
})

const handl = new NavigableWithHandlers(getParentNavigable(), containerProps())
setChildrenNavigable(handl)

afterUpdate(() => handl.updateProps(containerProps()))

export const requestFocus = () => handl.requestFocus()
</script>

<InternalNavWrapper navId={handl.id}>
  <slot nav={handl} />
</InternalNavWrapper>
