<script lang="ts">
  import { usePageNavigator, setChildrenNavigable } from '../../navigable/navigation'
  import AlbumGrid from '../AlbumGrid/AlbumGrid.svelte'
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'

  const nav = usePageNavigator()

  setChildrenNavigable(nav)
</script>

<NavigableList>
  <AlbumGrid />
</NavigableList>
