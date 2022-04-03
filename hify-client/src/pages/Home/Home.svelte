<script lang="ts">
  import AlbumGrid from '../AlbumsGrid/AlbumsGrid.svelte'
  import NavigableWithHandlers from '../../navigable/NavigableWithHandlers/NavigableWithHandlers.svelte'
  import { useNavigate } from 'svelte-navigator'

  const navigate = useNavigate()
</script>

<AlbumGrid />
