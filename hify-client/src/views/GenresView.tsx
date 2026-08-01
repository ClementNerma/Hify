import { useSuspensePaginatedQuery } from '#/api/hooks.ts'
import { fetchGenres } from '#/api/queries.ts'
import { GenreCard } from '#/components/molecules/GenreCard.tsx'
import { NavGrid } from '#/components/navigables/Grid.tsx'
import { noParallel } from '../utils/common'

export function GenresView() {
  const COLUMNS = 9
  const ROWS_PER_PAGE = 10

  const { data: genres, fetchNextPage } = useSuspensePaginatedQuery({
    query: (pagination) => fetchGenres({ sortBy: 'NAME', ...pagination }),
    paginationDir: 'ASC',
    pageSize: ROWS_PER_PAGE * COLUMNS,
  })

  return (
    <NavGrid
      items={genres}
      keyOfItem={(item) => item.genre.id}
      columns={COLUMNS}
      fetchMore={{
        rowsEagerness: ROWS_PER_PAGE / 2,
        debouncedLoader: noParallel(fetchNextPage),
      }}
    >
      {({ genre }) => <GenreCard genre={genre} />}
    </NavGrid>
  )
}
