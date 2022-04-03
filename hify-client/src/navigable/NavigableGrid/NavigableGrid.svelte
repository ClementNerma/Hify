<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableGrid } from './NavigableGrid'

  export let columns: number
  export let lazy: (() => void) | undefined = undefined

  const nav = getParentNavigable()
  const row = new NavigableGrid(nav, columns, lazy)

  nav.append(row)

  setChildrenNavigable(row)

  onDestroy(() => nav.remove(row))
</script>

<slot />
