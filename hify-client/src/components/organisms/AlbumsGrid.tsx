import { useState } from 'react'
import {
  FaArrowDownAZ,
  FaClock,
  FaStar,
  FaRegStar,
  FaHourglass,
  FaArrowDown91,
  FaArrowDownWideShort,
  FaArrowDownShortWide,
} from 'react-icons/fa6'
import { useSuspensePaginatedQuery, type CachableQuery } from '#/api/hooks.ts'
import type {
  AlbumCompleteInfos,
  AlbumsSort,
  Paginated,
  Pagination,
  PaginationDir,
  UserMixSource,
} from '#/api/types.ts'
import { openContextMenu } from '#/global/ctx-menu.tsx'
import { playNewMix } from '#/global/player.ts'
import { randomInt } from '#/utils/common.ts'
import { Button } from '../atoms/Button'
import { AlbumCard } from '../molecules/AlbumCard'
import { NavGrid } from '../navigables/Grid'
import { NavRow } from '../navigables/Row'
import { Select } from './Select'

export type AlbumsGridProps = {
  query: (
    sortBy: AlbumsSort,
    pagination: Pagination,
  ) => CachableQuery<Paginated<AlbumCompleteInfos>>
  mixSource: UserMixSource
}

export function AlbumsGrid({ query, mixSource }: AlbumsGridProps) {
  const COLUMNS = 7

  const [sortBy, setSortBy] = useState<AlbumsSort>('ADDED')
  const [paginationDir, setPaginationDir] = useState<PaginationDir>(defaultPaginationOrder[sortBy])

  const {
    data: albums,
    fetchNextPage,
    isResetting,
  } = useSuspensePaginatedQuery({
    query: (pagination) => query(sortBy, pagination),
    paginationDir,
    pageSize: 10 * COLUMNS,
  })

  return (
    <>
      <NavRow>
        <Select
          items={[
            { icon: <FaArrowDownAZ />, label: 'Name', value: 'NAME' },
            { icon: <FaClock />, label: 'Added date', value: 'ADDED' },
            { icon: <FaClock />, label: 'Date', value: 'DATE' },
            {
              icon: <FaStar />,
              label: 'Most great tracks',
              value: 'BEST_TRACKS_COUNT',
            },
            { icon: <FaRegStar />, label: 'Unrated first', value: 'UNRATED_FIRST' },
            { icon: <FaStar />, label: 'Rated tracks count', value: 'RATED_TRACKS_COUNT' },
            { icon: <FaHourglass />, label: 'Duration', value: 'DURATION' },
            { icon: <FaArrowDown91 />, label: 'Tracks count', value: 'TRACKS_COUNT' },
          ]}
          initialValue={sortBy}
          onSelect={(value) => {
            setSortBy(value)
            setPaginationDir(defaultPaginationOrder[value])
          }}
        />

        <Button onPress={() => setPaginationDir(paginationDir === 'ASC' ? 'DESC' : 'ASC')}>
          {paginationDir === 'ASC' ? <FaArrowDownShortWide /> : <FaArrowDownWideShort />} Reverse
        </Button>

        <Button
          onPress={() => playNewMix({ source: mixSource, filter: 'WELL_RATED', seed: randomInt() })}
          // oxlint-disable-next-line react/no-unstable-nested-components
          onLongPress={() =>
            openContextMenu([
              {
                icon: <FaStar />,
                label: 'Only the best ones',
                onPress() {
                  playNewMix({ source: mixSource, filter: 'BEST_RATED', seed: randomInt() })
                },
                skipFocusRestore: true,
              },
            ])
          }
        >
          Play great tracks ✨
        </Button>
      </NavRow>

      <div className="h-4" />

      <NavGrid
        columns={COLUMNS}
        items={albums}
        keyOfItem={(item) => item.album.id}
        onLastRow={fetchNextPage}
        className={`h-[calc(100vh-9rem)] overflow-y-auto ${isResetting ? 'opacity-50' : ''}`}
      >
        {(item) => <AlbumCard album={item} />}
      </NavGrid>
    </>
  )
}

const defaultPaginationOrder: Record<AlbumsSort, PaginationDir> = {
  NAME: 'ASC',
  ADDED: 'DESC',
  DATE: 'DESC',
  BEST_TRACKS_COUNT: 'DESC',
  UNRATED_FIRST: 'ASC',
  RATED_TRACKS_COUNT: 'DESC',
  DURATION: 'DESC',
  TRACKS_COUNT: 'DESC',
}
