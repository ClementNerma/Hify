import { ContextMenuOption } from '@navigable/ui/molecules/ContextMenu/ContextMenu'
import { ROUTES } from '@root/routes'
import { navigate } from 'svelte-navigator'

export const ctxMenuOptions = {
	goToAlbum(albumId: string): ContextMenuOption {
		return { label: 'Go to album', onPress: () => navigate(ROUTES.album(albumId)) }
	},
}
