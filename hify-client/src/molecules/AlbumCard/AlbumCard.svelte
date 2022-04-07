<script lang="ts">
  import { useNavigate } from 'svelte-navigator'
  import { AlbumInfos, ArtistInfos } from '../../graphql/generated'
  import { getAlbumArtUri } from '../../rest-api'
  import { ROUTES } from '../../routes'
  import Card from '../Card/Card.svelte'

  export let album: Pick<AlbumInfos, 'id' | 'name'> & {
    albumArtists: Array<Pick<ArtistInfos, 'name'>>
  }

  const navigate = useNavigate()
</script>

<Card
  title={album.name}
  subtitle={album.albumArtists.map((artist) => artist.name).join(', ')}
  onPress={() => navigate(ROUTES.album(album.id))}
  pictureUrl={getAlbumArtUri(album.id)}
/>
