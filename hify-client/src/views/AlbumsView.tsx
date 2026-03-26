import { fetchAlbums } from '#/api/queries.ts'
import { AlbumsGrid } from '#/components/organisms/AlbumsGrid.tsx'

export function AlbumsView() {
  return (
    <AlbumsGrid
      queryKey={['albums']}
      queryFn={(sortBy, pagination) => fetchAlbums({ sortBy, ...pagination })}
      // TODO: remove (doesn't make much sense for this view)
      mixSource={{ type: 'all' }}
    />
  )
}
