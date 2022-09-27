<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, OnFocusChangeCallback, setChildrenNavigable } from '../navigation'
  import { NavigableOne } from './NavigableOne'

  export let position: number | null = null
  export let hasFocusPriority: boolean | null = null

  export let whenFocusChanges: OnFocusChangeCallback | null = null

  const nav = getParentNavigable()
  const one = new NavigableOne(nav, position, hasFocusPriority)
  one.onFocusChangeCallback = whenFocusChanges

  nav.append(one)

  setChildrenNavigable(one)

  afterUpdate(() => {
    one.position = position
    one.hasFocusPriority = hasFocusPriority
    one.onFocusChangeCallback = whenFocusChanges
  })

  onDestroy(() => nav.remove(one))

  export const requestFocus = () => one.requestFocus()
</script>

<slot nav={one} />
