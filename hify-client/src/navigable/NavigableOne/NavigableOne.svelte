<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableOne } from './NavigableOne'

  export let position: number | null = null
  export let hasFocusPriority: boolean | null = null

  const nav = getParentNavigable()
  const row = new NavigableOne(nav, position, hasFocusPriority)

  nav.append(row)

  setChildrenNavigable(row)

  afterUpdate(() => {
    row.position = position
    row.hasFocusPriority = hasFocusPriority
  })

  onDestroy(() => nav.remove(row))
</script>

<slot nav={row} />
