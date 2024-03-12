<script lang="ts">
import { ctxMenuCallbacks, ctxMenuOptions } from '@globals/context-menu-items'
import { AlbumCardFragment } from '@graphql/generated'
import Card from '@molecules/Card/Card.svelte'
import ProgressiveRow from '@molecules/ProgressiveRow/ProgressiveRow.svelte'
import { ROUTES } from '@root/routes'
import { navigate } from 'svelte-navigator'

export let albums: AlbumCardFragment[]
</script>

<ProgressiveRow
  items={albums}
  idProp="id"
  onItemPress={(album) => navigate(ROUTES.album(album.id))}
  onItemLongPress={() => {
    // TODO: context menu
  }}
  let:item={album}
>
  <!-- TODO: enforceMaxWidth? -->
  <Card title={album.name} subtitle={album.albumArtists.map((artist) => artist.name).join(', ')} art={album.art} />
</ProgressiveRow>
