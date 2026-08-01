import { useSuspenseQuery } from '#/api/hooks.ts'
import { TrackCard } from '#/components/molecules/TrackCard.tsx'
import { NavGrid } from '#/components/navigables/Grid.tsx'
import { loadPersistentData, tryFetchHistoryTracks } from '#/global/persistent.ts'
import { playTrackFromNewQueue } from '#/global/player.ts'

export function HistoryView() {
  const COLUMNS = 7

  const { historyTrackIds } = loadPersistentData()

  const historyTracks = useSuspenseQuery(tryFetchHistoryTracks(historyTrackIds))

  if (historyTracks.length === 0) {
    return <h1 className="fixed top-1/3 w-full text-center">History is empty</h1>
  }

  return (
    <NavGrid items={historyTracks} keyOfItem={(item) => item.track.id} columns={COLUMNS}>
      {(track, i) => (
        <TrackCard
          track={track}
          onPress={() =>
            playTrackFromNewQueue(historyTracks, i, { gotoPlayer: true, fromMix: null })
          }
        />
      )}
    </NavGrid>
  )
}
