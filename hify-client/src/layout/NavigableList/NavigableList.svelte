<script lang="ts">
  import { getParentNavigable, setChildrenNavigable } from '../navigation'
  import { NavigableList } from './NavigableList'

  const nav = getParentNavigable()
  const list = new NavigableList(nav)

  nav.append(list)

  setChildrenNavigable(list)
</script>

<slot />
