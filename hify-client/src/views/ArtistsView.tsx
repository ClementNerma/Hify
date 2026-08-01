import { useSuspensePaginatedQuery } from '#/api/hooks.ts'
import { fetchArtistsWithAlbums } from '#/api/queries.ts'
import { ArtistCard } from '#/components/molecules/ArtistCard.tsx'
import { NavGrid } from '#/components/navigables/Grid.tsx'

// TODO: option to show artists without albums
export function ArtistsView() {
  const COLUMNS = 9

  const { data: artists, fetchNextPage } = useSuspensePaginatedQuery({
    query: (pagination) => fetchArtistsWithAlbums({ sortBy: 'NAME', ...pagination }),
    paginationDir: 'ASC',
    pageSize: 10 * COLUMNS,
  })

  return (
    <NavGrid
      items={artists}
      keyOfItem={(item) => item.artist.id}
      columns={COLUMNS}
      onLastRow={fetchNextPage}
    >
      {({ artist }) => <ArtistCard artist={artist} />}
    </NavGrid>
  )
}
