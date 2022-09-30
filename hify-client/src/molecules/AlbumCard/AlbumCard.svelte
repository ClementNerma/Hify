<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AlbumCardFragment } from '../../graphql/generated'
  import { getAlbumArtUri } from '../../globals/rest-api'
  import { ROUTES } from '../../routes'
  import { bind } from '../../globals/utils'
  import InteractiveCard from '../Card/InteractiveCard.svelte'

  export let album: AlbumCardFragment
  export let enforceMaxWidth = false
</script>

<InteractiveCard
  title={album.name}
  subtitle={album.albumArtists.map((artist) => artist.name).join(', ')}
  onPress={bind(album, (album) => navigate(ROUTES.album(album.id)))}
  onLongPress={() => alert('TODO: context menu for playing options')}
  pictureUrl={getAlbumArtUri(album.id)}
  {enforceMaxWidth}
/>
