<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableList } from './NavigableList'

  export let position: number | null = null
  export let hasFocusPriority: boolean | null = null

  const nav = getParentNavigable()
  const list = new NavigableList(nav, position, hasFocusPriority)

  nav.append(list)

  setChildrenNavigable(list)

  afterUpdate(() => {
    list.position = position
    list.hasFocusPriority = hasFocusPriority
  })

  onDestroy(() => nav.remove(list))
</script>

<slot nav={list} />
