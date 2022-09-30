<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, Props, setChildrenNavigable } from '../navigation'
  import { NavigableWithHandlers } from './NavigableWithHandlers'

  export let position: Props<NavigableWithHandlers>['position'] = null
  export let hasFocusPriority: Props<NavigableWithHandlers>['hasFocusPriority'] = null

  export let onPress: Props<NavigableWithHandlers>['onPress'] = undefined
  export let onLongPress: Props<NavigableWithHandlers>['onLongPress'] = undefined
  export let onBack: Props<NavigableWithHandlers>['onBack'] = undefined
  export let onLongBack: Props<NavigableWithHandlers>['onLongBack'] = undefined
  export let onKeyPress: Props<NavigableWithHandlers>['onKeyPress'] = undefined

  const containerProps = (): Props<NavigableWithHandlers> => ({
    position,
    hasFocusPriority,
    onPress,
    onLongPress,
    onBack,
    onLongBack,
    onKeyPress,
  })

  const nav = getParentNavigable()
  const handl = new NavigableWithHandlers(nav, containerProps())

  nav.append(handl)

  setChildrenNavigable(handl)

  afterUpdate(() => handl.updateProps(containerProps()))

  onDestroy(() => nav.remove(handl))

  export const requestFocus = () => handl.requestFocus()
</script>

<slot nav={handl} />
