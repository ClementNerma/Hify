import { useSuspenseQuery } from '#/api/hooks.ts'
import { fetchMultiTracks } from '#/api/queries.ts'
import { TrackCard } from '#/components/molecules/TrackCard.tsx'
import { NavGrid } from '#/components/navigables/Grid.tsx'
import { loadPersistentData } from '#/global/persistent.ts'
import { playTrackFromNewQueue } from '#/global/player.ts'

export function HistoryView() {
  const { historyTrackIds } = loadPersistentData()

  const historyTracks = useSuspenseQuery({
    queryKey: ['tracks', historyTrackIds.join(',')],
    queryFn: () => fetchMultiTracks(historyTrackIds),
  })

  if (historyTracks.length === 0) {
    return <h1 className="fixed top-1/3 w-full text-center">History is empty</h1>
  }

  const COLUMNS = 7

  return (
    <NavGrid columns={COLUMNS}>
      {/* TODO: make grid-cols-7 dynamic with `COLUMNS` */}
      <div className="grid grid-cols-7 auto-rows-fr gap-4">
        {Array.from({ length: Math.ceil(historyTracks.length / COLUMNS) }).map((_, rowIndex) => {
          const rowTracks = historyTracks.slice(rowIndex * COLUMNS, rowIndex * COLUMNS + COLUMNS)

          return rowTracks.map((track, i) => (
            <div key={track.track.id} className="flex">
              <TrackCard
                track={track}
                onPress={() =>
                  playTrackFromNewQueue(historyTracks, i, { gotoPlayer: true, fromMix: null })
                }
              />
            </div>
          ))
        })}
      </div>
    </NavGrid>
  )
}
