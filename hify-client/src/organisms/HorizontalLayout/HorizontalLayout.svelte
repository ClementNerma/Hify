<script lang="ts">
  import { getParentNavigable, setChildrenNavigable } from '../../others/navigation'
  import { NavigableRow } from '../NavigableRow/NavigableRow'

  const nav = getParentNavigable()
  const row = new NavigableRow(nav)

  nav.append(row)

  setChildrenNavigable(row)
</script>

<slot />
