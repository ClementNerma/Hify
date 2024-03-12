<script lang="ts">
import { navigate } from 'svelte-navigator'
import { AlbumCardFragment } from '@graphql/generated'
import { ROUTES } from '@root/routes'
import { bind } from '@globals/utils'
import InteractiveCard from '../Card/InteractiveCard.svelte'
import { SimpleNavigableItemProps } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem'

export let album: AlbumCardFragment
export let enforceMaxWidth = false
export let onFocus: SimpleNavigableItemProps['onFocus'] = undefined
</script>

<InteractiveCard
  title={album.name}
  subtitle={album.albumArtists.map((artist) => artist.name).join(', ')}
  onPress={bind(album, (album) => navigate(ROUTES.album(album.id)))}
  onLongPress={() => alert('TODO: context menu for playing options')}
  art={album.art}
  {enforceMaxWidth}
  {onFocus}
/>
