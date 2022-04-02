<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableGrid } from './NavigableGrid'

  export let columns: number

  const nav = getParentNavigable()
  const row = new NavigableGrid(nav, columns)

  nav.append(row)

  setChildrenNavigable(row)

  onDestroy(() => nav.remove(row))
</script>

<slot />
