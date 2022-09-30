import { navigate } from 'svelte-navigator'
import { ContextMenuOption } from '../navigable/ui/molecules/ContextMenu/ContextMenu'
import { ROUTES } from '../routes'

export const ctxMenuOptions = {
  goToAlbum(albumId: string): ContextMenuOption {
    return { label: 'Go to album', onPress: () => navigate(ROUTES.album(albumId)) }
  },
}
