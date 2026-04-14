import { useState } from 'react'
import { FaClock, FaCompactDisc, FaHourglass, FaMicrophoneLines, FaPlay } from 'react-icons/fa6'
import { useSuspenseQueries } from '#/api/hooks.ts'
import { fetchAlbum, fetchAlbumTracks } from '#/api/queries.ts'
import type { TrackCompleteInfos } from '#/api/types.ts'
import { urls } from '#/api/urls.ts'
import { Button } from '#/components/atoms/Button.tsx'
import { CheckBox } from '#/components/molecules/CheckBox.tsx'
import { OneLineList } from '#/components/molecules/OneLineList.tsx'
import { RatingDisplay } from '#/components/molecules/RatingDisplay.tsx'
import { BlockNavItem } from '#/components/navigables/Item.tsx'
import { NavList } from '#/components/navigables/List.tsx'
import { NavRow } from '#/components/navigables/Row.tsx'
import { defaultCtxMenus, openContextMenu } from '#/global/ctx-menu.tsx'
import { enqueueTracksAsNext, playTrackFromNewQueue } from '#/global/player.ts'
import { navigate } from '#/router/routes.ts'
import { routes } from '#/routes.ts'
import { filterMap, formatDuration, unwrapNotUndefined } from '#/utils/common.ts'

export type AlbumViewProps = {
  albumId: string
}

export function AlbumView({ albumId }: AlbumViewProps) {
  const [{ album, artists, genres }, allTracks] = useSuspenseQueries(
    {
      queryKey: ['album', albumId],
      queryFn: () => fetchAlbum(albumId),
    },
    {
      queryKey: ['albumTracks', albumId],
      queryFn: () => fetchAlbumTracks(albumId),
    },
  )

  const [tracksFilter, setTracksFilter] = useState<'great' | 'best' | null>(null)

  const tracks =
    tracksFilter === null
      ? allTracks
      : allTracks.filter(
          ({ rating }) => rating === 'Five' || (tracksFilter === 'great' && rating === 'Four'),
        )

  const tracksByDisc = Object.entries(
    Object.groupBy(tracks, ({ track }) => track.tags.discNumber ?? '<unknown>'),
  )

  const albumYears = computeAlbumYears(tracks)

  return (
    <NavList className="mx-64" focusChildIndexOnEnter={1}>
      <div className="flex flex-row gap-3">
        <img src={urls.albumArt(album, 'medium')} width={250} height={250} />

        <NavList className="text-lg">
          <NavRow>
            <OneLineList
              className="p-1.5"
              items={artists.map(({ artist }) => ({
                label: (
                  <>
                    <FaMicrophoneLines /> {artist.name}
                  </>
                ),
                value: artist.id,
              }))}
              onSelect={(artistId) => navigate(routes.artist, { artistId })}
            />

            <OneLineList
              className="p-1.5"
              items={genres.map(({ genre }) => ({
                label: (
                  <>
                    <FaMicrophoneLines /> {genre.name}
                  </>
                ),
                value: genre.id,
              }))}
              onSelect={(genreId) => navigate(routes.genre, { genreId })}
            />
          </NavRow>

          <NavRow>
            <CheckBox
              className="p-1.5"
              checked={tracksFilter === 'great'}
              onChange={(checked) => setTracksFilter(checked ? 'great' : null)}
            >
              Great tracks
            </CheckBox>

            <CheckBox
              className="p-1.5"
              checked={tracksFilter === 'best'}
              onChange={(checked) => setTracksFilter(checked ? 'best' : null)}
            >
              Best tracks
            </CheckBox>

            <Button onPress={() => enqueueTracksAsNext(tracks)}>
              <FaPlay /> Play next
            </Button>
          </NavRow>

          <div className="p-1.5">
            <FaClock />{' '}
            {albumYears
              ? albumYears.firstYear === albumYears.lastYear
                ? albumYears.firstYear
                : `${albumYears.firstYear} - ${albumYears.lastYear}`
              : 'Unknown'}
          </div>

          <div className="p-1.5">
            <FaCompactDisc /> {tracks.length} track{tracks.length > 1 ? 's' : ''}
          </div>

          <div className="p-1.5">
            <FaHourglass />{' '}
            {formatDuration(tracks.reduce((sum, t) => sum + t.track.metadata.durationS, 0))}
          </div>
        </NavList>
      </div>

      {tracksByDisc.map(([disc, discTracks]) => (
        <div key={disc}>
          {tracksByDisc.length > 1 && <h2>Disc {disc}</h2>}

          <div className="grid grid-cols-[max-content_1fr_max-content_max-content] mt-2">
            {unwrapNotUndefined(discTracks).map((track) => (
              <BlockNavItem
                key={track.track.id}
                className="grid grid-cols-subgrid col-span-full p-2 gap-0 not-last:border-b-gray-700 *:p-1"
                onPress={() =>
                  playTrackFromNewQueue(
                    tracks,
                    tracks.findIndex((c) => c.track.id === track.track.id),
                    { gotoPlayer: true, fromMix: null },
                  )
                }
                onLongPress={() => openContextMenu(defaultCtxMenus.track(track))}
              >
                <span className="mr-3 align-top text-center">
                  {track.track.tags.trackNumber ?? ''}{' '}
                </span>
                <span>{track.track.tags.title}</span>
                <span className="mr-4">
                  {track.rating && <RatingDisplay rating={track.rating} />}
                </span>
                <span className="relative top-0.5">
                  {formatDuration(track.track.metadata.durationS)}
                </span>
              </BlockNavItem>
            ))}
          </div>
        </div>
      ))}
    </NavList>
  )
}

function computeAlbumYears(
  tracks: TrackCompleteInfos[],
): { firstYear: number; lastYear: number } | null {
  const trackYears = filterMap(tracks, ({ track }) => track.tags.date).map((date) => date.year)

  if (trackYears.length === 0) {
    return null
  }

  return { firstYear: Math.min(...trackYears), lastYear: Math.max(...trackYears) }
}
