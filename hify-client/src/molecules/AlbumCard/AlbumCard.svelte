<script lang="ts">
  import { useNavigate } from 'svelte-navigator'
  import { AlbumInfos, ArtistInfos } from '../../graphql/types'
  import { getAlbumArtUri } from '../../rest-api'
  import { ROUTES } from '../../routes'
  import Card from '../Card/Card.svelte'

  export let album: Pick<AlbumInfos, 'id' | 'name'> & {
    albumArtists: Array<Pick<ArtistInfos, 'name'>>
  }

  const navigate = useNavigate()
</script>

<Card
  _key={album.id}
  title={album.name}
  subtitle={album.albumArtists.map((artist) => artist.name).join(', ')}
  onPress={(id) => navigate(ROUTES.album(id))}
  pictureUrl={getAlbumArtUri(album.id)}
  pictureAlt={album.name}
/>
