import { useSuspenseQueries } from '#/api/hooks.ts'
import { fetchArtist, fetchArtistAlbumParticipations, fetchArtistAlbums } from '#/api/queries.ts'
import { AlbumCard } from '#/components/molecules/AlbumCard.tsx'
import { NavRow } from '#/components/navigables/Row.tsx'

type ArtistViewProps = { artistId: string }

export function ArtistView({ artistId }: ArtistViewProps) {
  const [{ artist }, albumsByName, /*albumsByDate, albumsByScore,*/ albumParticipationsByName] =
    useSuspenseQueries(
      {
        queryKey: ['artist', artistId],
        queryFn: () => fetchArtist(artistId),
      },
      {
        queryKey: ['artistAlbums', artistId],
        queryFn: () => fetchArtistAlbums(artistId, 'DATE', { dir: 'DESC', limit: 50, offset: 0 }),
      },
      {
        queryKey: ['artistAlbumParticipations', artistId],
        queryFn: () =>
          fetchArtistAlbumParticipations(artistId, 'DATE', {
            limit: 50,
            offset: 0,
            dir: 'DESC',
          }),
      },
    )

  return (
    <div>
      <h1 className="text-center">{artist.name}</h1>

      {albumsByName.total > 0 && (
        <>
          <h2>Albums ({albumsByName.total})</h2>

          <NavRow className="items-row-auto">
            {albumsByName.results.map((album) => (
              <AlbumCard key={album.album.id} album={album} />
            ))}
          </NavRow>
        </>
      )}

      {albumParticipationsByName.total > 0 && (
        <>
          <h2>Album participations ({albumParticipationsByName.total})</h2>

          <NavRow className="items-row-auto">
            {albumParticipationsByName.results.map((album) => (
              <AlbumCard key={album.album.id} album={album} />
            ))}
          </NavRow>
        </>
      )}
    </div>
  )
}
