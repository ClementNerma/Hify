import type { TrackCompleteInfos } from '#/api/types.ts'
import { urls } from '#/api/urls.ts'
import { NavItem, type NavItemProps } from '#/components/navigables/Item.tsx'
import { defaultCtxMenus, openContextMenu } from '#/global/ctx-menu.tsx'
import type { ContextMenuItem } from '../organisms/ContextMenu'
import { Card } from './Card'

export type TrackCardProps = {
  track: TrackCompleteInfos
  replaceCtxMenu?: ContextMenuItem[]
} & Omit<NavItemProps, 'onLongPress'>

export function TrackCard({
  track: completeTrack,
  replaceCtxMenu,
  ...navItemProps
}: TrackCardProps) {
  const { track, album } = completeTrack

  return (
    <NavItem
      {...navItemProps}
      onLongPress={() => openContextMenu(replaceCtxMenu ?? defaultCtxMenus.track(completeTrack))}
    >
      <Card title={track.tags.title} artUrl={urls.albumArt(album.album, 'small')} />
    </NavItem>
  )
}
