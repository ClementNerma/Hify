import { navigate } from 'svelte-navigator'
import { ContextMenuOption } from '../molecules/ContextMenu/ContextMenu.svelte'
import { ROUTES } from '../routes'

export const ctxMenuOptions = {
  goToAlbum(albumId: string): ContextMenuOption {
    return { label: 'Go to album', onPress: () => navigate(ROUTES.album(albumId)) }
  },
}
