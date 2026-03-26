import type { AlbumCompleteInfos } from '#/api/types.ts'
import { urls } from '#/api/urls.ts'
import { NavItem } from '#/components/navigables/Item.tsx'
import { defaultCtxMenus, openContextMenu } from '#/global/ctx-menu.tsx'
import { navigate } from '#/router/routes.ts'
import { routes } from '#/routes.ts'
import { Card } from './Card'

export type AlbumCardProps = {
  album: AlbumCompleteInfos
}

export function AlbumCard({ album }: AlbumCardProps) {
  return (
    <NavItem
      onPress={() => navigate(routes.album, { albumId: album.album.id })}
      onLongPress={() => openContextMenu(defaultCtxMenus.album(album))}
    >
      <Card title={album.album.name} artUrl={urls.albumArt(album.album, 'small')} />
    </NavItem>
  )
}
