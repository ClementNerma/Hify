<script lang="ts">
  import { afterUpdate, onDestroy } from 'svelte'

  import { getParentNavigable, Props, setChildrenNavigable } from '../navigation'
  import { NavigableList } from './NavigableList'

  export let position: Props<NavigableList>['position'] = null
  export let hasFocusPriority: Props<NavigableList>['hasFocusPriority'] = null

  const nav = getParentNavigable()
  const list = new NavigableList(nav, { position, hasFocusPriority })

  nav.append(list)

  setChildrenNavigable(list)

  afterUpdate(() => list.updateProps({ position, hasFocusPriority }))

  onDestroy(() => nav.remove(list))

  export const requestFocus = () => list.requestFocus()
</script>

<slot nav={list} />
