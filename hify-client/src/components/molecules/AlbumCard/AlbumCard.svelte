<script lang="ts">
import { bind } from '@globals/utils'
import type { AlbumCardFragment } from '@graphql/generated'
import type { SimpleNavigableItemProps } from '@navigable/headless/SimpleNavigableItem/SimpleNavigableItem'
import { ROUTES } from '@root/routes'
import { navigate } from 'svelte-navigator'
import InteractiveCard from '../Card/InteractiveCard.svelte'

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
