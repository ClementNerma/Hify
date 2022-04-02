<script lang="ts">
  import { getParentNavigable, setChildrenNavigable } from '../../others/navigation'
  import { NavigableList } from './NavigableList'

  const nav = getParentNavigable()
  const list = new NavigableList(nav)

  nav.append(list)

  setChildrenNavigable(list)
</script>

<slot />
