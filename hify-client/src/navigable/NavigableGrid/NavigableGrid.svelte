<script lang="ts">
  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableGrid } from './NavigableGrid'

  export let columns: number

  const nav = getParentNavigable()
  const row = new NavigableGrid(nav, columns)

  nav.append(row)

  setChildrenNavigable(row)
</script>

<slot />
