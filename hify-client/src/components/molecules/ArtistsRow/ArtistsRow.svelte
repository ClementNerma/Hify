<script lang="ts">
  import { ctxMenuCallbacks, ctxMenuOptions } from '@globals/context-menu-items'
  import { ArtistCardFragment } from '@graphql/generated'
  import Card from '@molecules/Card/Card.svelte'
  import ProgressiveRow from '@molecules/ProgressiveRow/ProgressiveRow.svelte'
  import { showContextMenu } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
  import { ROUTES } from '@root/routes'
  import { navigate } from 'svelte-navigator'

  export let artists: ArtistCardFragment[]
</script>

<ProgressiveRow
  initialItems={artists}
  idProp="id"
  onItemPress={(artist) => navigate(ROUTES.artist(artist.id))}
  onItemLongPress={(artist) => {
    showContextMenu(ctxMenuOptions.forArtist(artist.id))
  }}
  let:item={artist}
>
  <!-- TODO: enforceMaxWidth? -->
  <Card title={artist.name} subtitle="" art={artist.art} />
</ProgressiveRow>
