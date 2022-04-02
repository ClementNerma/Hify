<script lang="ts">
  import HorizontalLayout from '../../organisms/HorizontalLayout/HorizontalLayout.svelte'

  import { setChildrenNavigable, usePageNavigator } from '../../others/navigation'

  import AlbumGrid from '../AlbumGrid/AlbumGrid.svelte'

  const nav = usePageNavigator()

  setChildrenNavigable(nav)
</script>

<HorizontalLayout>
  <AlbumGrid />
</HorizontalLayout>
