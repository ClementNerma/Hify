<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableOne } from './NavigableOne'

  export let position: (() => number) | null = null
  export let hasFocusPriority: (() => boolean) | null = null

  const nav = getParentNavigable()
  const row = new NavigableOne(nav, position, hasFocusPriority)

  nav.append(row)

  setChildrenNavigable(row)

  onDestroy(() => nav.remove(row))
</script>

<slot nav={row} />
