<script lang="ts">
  import { navigate } from 'svelte-navigator'
  import { AlbumCardFragment } from '../../graphql/generated'
  import { getAlbumArtUri } from '../../rest-api'
  import { ROUTES } from '../../routes'
  import { bind } from '../../utils'
  import InteractiveCard from '../Card/InteractiveCard.svelte'

  export let album: AlbumCardFragment
</script>

<InteractiveCard
  title={album.name}
  subtitle={album.albumArtists.map((artist) => artist.name).join(', ')}
  onPress={bind(album, (album) => navigate(ROUTES.album(album.id)))}
  pictureUrl={getAlbumArtUri(album.id)}
/>
