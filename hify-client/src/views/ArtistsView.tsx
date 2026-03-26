import { useSuspensePaginatedQuery } from '#/api/hooks.ts'
import { fetchArtistsWithAlbums } from '#/api/queries.ts'
import { ArtistCard } from '#/components/molecules/ArtistCard.tsx'
import { NavGrid } from '#/components/navigables/Grid.tsx'

// TODO: option to show artsits without albums
export function ArtistsView() {
  const COLUMNS = 9

  const { data: artists, fetchNextPage } = useSuspensePaginatedQuery({
    queryKey: ['artists'],
    queryFn: (pagination) => fetchArtistsWithAlbums({ sortBy: 'NAME', ...pagination }),
    paginationDir: 'ASC',
    pageSize: 10 * COLUMNS,
  })

  return (
    <NavGrid columns={COLUMNS} onLastRow={fetchNextPage}>
      {/* TODO: make grid-cols-9 dynamic with `COLUMNS` */}
      <div className="grid grid-cols-9 auto-rows-fr gap-4">
        {Array.from({ length: Math.ceil(artists.length / COLUMNS) }).map((_, rowIndex) => {
          const rowArtists = artists.slice(rowIndex * COLUMNS, rowIndex * COLUMNS + COLUMNS)

          return rowArtists.map(({ artist }) => (
            <div key={artist.id} className="flex [&_img]:rounded-full">
              <ArtistCard artist={artist} />
            </div>
          ))
        })}
      </div>
    </NavGrid>
  )
}
