<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableWithHandlers, NavigableWithHandlersProps } from './NavigableWithHandlers'

  export let position: NavigableWithHandlersProps['position'] = null
  export let hasFocusPriority: NavigableWithHandlersProps['hasFocusPriority'] = null

  export let onPress: NavigableWithHandlersProps['onPress'] = undefined
  export let onLongPress: NavigableWithHandlersProps['onLongPress'] = undefined
  export let onBack: NavigableWithHandlersProps['onBack'] = undefined

  const nav = getParentNavigable()
  const row = new NavigableWithHandlers(nav, { position, hasFocusPriority, onPress, onLongPress, onBack })

  nav.append(row)

  setChildrenNavigable(row)

  afterUpdate(() => {
    row.position = position
    row.hasFocusPriority = hasFocusPriority
  })

  onDestroy(() => nav.remove(row))
</script>

<slot nav={row} />
