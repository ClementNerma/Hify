import { useSuspensePaginatedQuery } from '#/api/hooks.ts'
import { fetchGenres } from '#/api/queries.ts'
import { GenreCard } from '#/components/molecules/GenreCard.tsx'
import { NavGrid } from '#/components/navigables/Grid.tsx'

export function GenresView() {
  const COLUMNS = 7

  const { data: genres, fetchNextPage } = useSuspensePaginatedQuery({
    query: (pagination) => fetchGenres({ sortBy: 'NAME', ...pagination }),
    paginationDir: 'ASC',
    pageSize: 10 * COLUMNS,
  })

  return (
    <NavGrid
      items={genres}
      keyOfItem={(item) => item.genre.id}
      columns={COLUMNS}
      onLastRow={fetchNextPage}
    >
      {({ genre }) => <GenreCard genre={genre} />}
    </NavGrid>
  )
}
