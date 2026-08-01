import { useSuspensePaginatedQuery } from '#/api/hooks.ts'
import { fetchArtistsWithAlbums } from '#/api/queries.ts'
import { ArtistCard } from '#/components/molecules/ArtistCard.tsx'
import { NavGrid } from '#/components/navigables/Grid.tsx'
import { noParallel } from '../utils/common'

// TODO: option to show artists without albums
export function ArtistsView() {
  const COLUMNS = 9
  const ROWS_PER_PAGE = 10

  const { data: artists, fetchNextPage } = useSuspensePaginatedQuery({
    query: (pagination) => fetchArtistsWithAlbums({ sortBy: 'NAME', ...pagination }),
    paginationDir: 'ASC',
    pageSize: ROWS_PER_PAGE * COLUMNS,
  })

  return (
    <NavGrid
      items={artists}
      keyOfItem={(item) => item.artist.id}
      columns={COLUMNS}
      fetchMore={{
        rowsEagerness: ROWS_PER_PAGE / 2,
        debouncedLoader: noParallel(fetchNextPage),
      }}
    >
      {({ artist }) => <ArtistCard artist={artist} />}
    </NavGrid>
  )
}
