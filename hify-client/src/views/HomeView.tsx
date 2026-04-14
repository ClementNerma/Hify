import { FaStar } from 'react-icons/fa6'
import { useSuspenseQueries } from '#/api/hooks.ts'
import { fetchAlbums, fetchMultiTracks } from '#/api/queries.ts'
import { Button } from '#/components/atoms/Button.tsx'
import { AlbumCard } from '#/components/molecules/AlbumCard.tsx'
import { TrackCard } from '#/components/molecules/TrackCard.tsx'
import { NavRow } from '#/components/navigables/Row.tsx'
import { openContextMenu } from '#/global/ctx-menu.tsx'
import { loadPersistentData } from '#/global/persistent.ts'
import { playNewMix, playTrackFromNewQueue } from '#/global/player.ts'
import { randomInt } from '#/utils/common.ts'

export function HomeView() {
  const { historyTrackIds } = loadPersistentData()

  const [historyTracks, newestAlbums] = useSuspenseQueries(
    {
      queryKey: ['history-tracks', historyTrackIds.join(',')],
      queryFn: () => fetchMultiTracks(historyTrackIds),
    },
    {
      queryKey: ['newest-albums'],
      queryFn: () => fetchAlbums({ sortBy: 'ADDED', dir: 'DESC', limit: 50, offset: null }),
    },
  )

  return (
    <>
      <h3 className="text-center">Welcome!</h3>

      <div className="flex justify-center">
        <Button
          onPress={() =>
            playNewMix({ filter: 'WELL_RATED', source: { type: 'all' }, seed: randomInt() })
          }
          onLongPress={() => {
            openContextMenu([
              {
                icon: <FaStar />,
                label: 'Only the best-rated songs',
                onPress() {
                  playNewMix({
                    filter: 'BEST_RATED',
                    source: { type: 'all' },
                    seed: randomInt(),
                  })
                },
                skipFocusRestore: true,
              },
            ])
          }}
        >
          Mix me some magic ✨
        </Button>
      </div>

      <h2 className="text-center">Last songs you listened to:</h2>

      <NavRow className="items-row-auto">
        {historyTracks.slice(0, 50).map((track, i) => (
          <TrackCard
            key={track.track.id}
            track={track}
            onPress={() =>
              playTrackFromNewQueue(historyTracks, i, { gotoPlayer: true, fromMix: null })
            }
          />
        ))}
      </NavRow>

      <h2 className="text-center">Latest albums:</h2>

      <NavRow className="items-row-auto">
        {newestAlbums.results.map((album) => (
          <AlbumCard key={album.album.id} album={album} />
        ))}
      </NavRow>
    </>
  )
}
