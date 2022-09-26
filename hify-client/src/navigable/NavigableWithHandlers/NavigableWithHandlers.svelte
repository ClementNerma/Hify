<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableWithHandlers, NavigableWithHandlersProps } from './NavigableWithHandlers'

  export let position: NavigableWithHandlersProps['position'] = null
  export let hasFocusPriority: NavigableWithHandlersProps['hasFocusPriority'] = null

  export let onPress: NavigableWithHandlersProps['onPress'] = undefined
  export let onLongPress: NavigableWithHandlersProps['onLongPress'] = undefined
  export let onBack: NavigableWithHandlersProps['onBack'] = undefined
  export let onLongBack: NavigableWithHandlersProps['onLongBack'] = undefined

  const containerProps = (): NavigableWithHandlersProps => ({
    position,
    hasFocusPriority,
    onPress,
    onLongPress,
    onBack,
    onLongBack,
  })

  const nav = getParentNavigable()
  const row = new NavigableWithHandlers(nav, containerProps())

  nav.append(row)

  setChildrenNavigable(row)

  afterUpdate(() => {
    row.position = position
    row.hasFocusPriority = hasFocusPriority
    row.props = containerProps()
  })

  onDestroy(() => nav.remove(row))

  export const requestFocus = () => row.requestFocus()
</script>

<slot nav={row} />
