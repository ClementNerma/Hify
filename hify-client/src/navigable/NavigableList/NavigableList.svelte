<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableList } from './NavigableList'

  export let position: (() => number) | null = null

  const nav = getParentNavigable()
  const list = new NavigableList(nav, position)

  nav.append(list)

  setChildrenNavigable(list)

  onDestroy(() => nav.remove(list))
</script>

<slot nav={list} />
