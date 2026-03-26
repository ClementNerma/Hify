import { useSuspensePaginatedQuery } from '#/api/hooks.ts'
import { fetchGenres } from '#/api/queries.ts'
import { GenreCard } from '#/components/molecules/GenreCard.tsx'
import { NavGrid } from '#/components/navigables/Grid.tsx'

export function GenresView() {
  const COLUMNS = 7

  const { data: genres, fetchNextPage } = useSuspensePaginatedQuery({
    queryKey: ['genres'],
    queryFn: (pagination) => fetchGenres({ sortBy: 'NAME', ...pagination }),
    paginationDir: 'ASC',
    pageSize: 10 * COLUMNS,
  })

  return (
    <NavGrid columns={COLUMNS} onLastRow={fetchNextPage}>
      {/* TODO: make grid-cols-9 dynamic with `COLUMNS` */}
      <div className="grid grid-cols-7 auto-rows-fr gap-4">
        {Array.from({ length: Math.ceil(genres.length / COLUMNS) }).map((_, rowIndex) => {
          const rowGenres = genres.slice(rowIndex * COLUMNS, rowIndex * COLUMNS + COLUMNS)

          return rowGenres.map(({ genre }) => (
            <div key={genre.id} className="flex [&_img]:rounded-full">
              <GenreCard genre={genre} />
            </div>
          ))
        })}
      </div>
    </NavGrid>
  )
}
