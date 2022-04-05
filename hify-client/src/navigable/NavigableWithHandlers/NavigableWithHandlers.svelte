<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, NavigableAttachedData, setChildrenNavigable } from '../navigation'
  import { NavigableWithHandlers, NavigableWithHandlersProps } from './NavigableWithHandlers'

  export let onPress: NavigableWithHandlersProps['onPress'] = undefined
  export let onLongPress: NavigableWithHandlersProps['onLongPress'] = undefined
  export let onBack: NavigableWithHandlersProps['onBack'] = undefined

  export let data: NavigableAttachedData = null

  const nav = getParentNavigable()
  const row = new NavigableWithHandlers(nav, { onPress, onLongPress, onBack, attachedData: data })

  nav.append(row)

  setChildrenNavigable(row)

  onDestroy(() => nav.remove(row))
</script>

<slot />
