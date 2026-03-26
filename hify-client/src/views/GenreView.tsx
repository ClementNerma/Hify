import { fetchGenreAlbums } from '#/api/queries.ts'
import { AlbumsGrid } from '#/components/organisms/AlbumsGrid.tsx'

export function GenreView({ genreId }: { genreId: string }) {
  return (
    <AlbumsGrid
      queryKey={['genreAlbums', genreId]}
      queryFn={(sortBy, pagination) => fetchGenreAlbums(genreId, { sortBy, ...pagination })}
      mixSource={{ type: 'genre', id: genreId }}
    />
  )
}
