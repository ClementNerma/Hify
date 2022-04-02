<script lang="ts">
  import HorizontalLayout from '../../layout/HorizontalLayout/HorizontalLayout.svelte'
  import { setChildrenNavigable, usePageNavigator } from '../../layout/navigation'

  import AlbumGrid from '../AlbumGrid/AlbumGrid.svelte'

  const nav = usePageNavigator()

  setChildrenNavigable(nav)
</script>

<HorizontalLayout>
  <AlbumGrid />
</HorizontalLayout>
