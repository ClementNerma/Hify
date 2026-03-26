import type { Genre } from '#/api/types.ts'
import { urls } from '#/api/urls.ts'
import { NavItem } from '#/components/navigables/Item.tsx'
import { navigate } from '#/router/routes.ts'
import { routes } from '#/routes.ts'
import { Card } from './Card'

export type GenreCardProps = {
  genre: Genre
}

export function GenreCard({ genre }: GenreCardProps) {
  return (
    <NavItem onPress={() => navigate(routes.genre, { genreId: genre.id })}>
      <Card title={genre.name} artUrl={urls.genreArt(genre, 'small')} />
    </NavItem>
  )
}
