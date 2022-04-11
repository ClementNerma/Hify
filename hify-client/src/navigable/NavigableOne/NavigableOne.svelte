<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableOne } from './NavigableOne'

  export let position: number | null = null
  export let hasFocusPriority: boolean | null = null

  const nav = getParentNavigable()
  const one = new NavigableOne(nav, position, hasFocusPriority)

  nav.append(one)

  setChildrenNavigable(one)

  afterUpdate(() => {
    one.position = position
    one.hasFocusPriority = hasFocusPriority
  })

  onDestroy(() => nav.remove(one))

  export const requestFocus = () => one.requestFocus()
</script>

<slot nav={one} />
