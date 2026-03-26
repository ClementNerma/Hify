import type { Artist } from '#/api/types.ts'
import { urls } from '#/api/urls.ts'
import { NavItem } from '#/components/navigables/Item.tsx'
import { navigate } from '#/router/routes.ts'
import { routes } from '#/routes.ts'
import { Card } from './Card'

export type ArtistCardProps = {
  artist: Artist
}

export function ArtistCard({ artist }: ArtistCardProps) {
  return (
    <NavItem onPress={() => navigate(routes.artist, { artistId: artist.id })}>
      <Card title={artist.name} artUrl={urls.artistArt(artist, 'small')} />
    </NavItem>
  )
}
