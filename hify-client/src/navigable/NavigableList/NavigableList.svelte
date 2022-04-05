<script lang="ts">
  import { onDestroy } from 'svelte'

  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableList } from './NavigableList'

  const nav = getParentNavigable()
  const list = new NavigableList(nav)

  nav.append(list)

  setChildrenNavigable(list)

  onDestroy(() => nav.remove(list))
</script>

<slot />
