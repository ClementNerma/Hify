<script lang="ts">
  import AlbumGrid from '../AlbumGrid/AlbumGrid.svelte'
  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import { useNavigate } from 'svelte-navigator'

  const navigate = useNavigate()
</script>

<NavigableWithHandlers onBack={() => navigate(-1)}>
  <AlbumGrid />
</NavigableWithHandlers>
