<script lang="ts">
  import { usePageNavigator, setChildrenNavigable } from '../../navigable/navigation'
  import HorizontalLayout from '../../navigable/HorizontalLayout/HorizontalLayout.svelte'
  import AlbumGrid from '../AlbumGrid/AlbumGrid.svelte'

  const nav = usePageNavigator()

  setChildrenNavigable(nav)
</script>

<HorizontalLayout>
  <AlbumGrid />
</HorizontalLayout>
