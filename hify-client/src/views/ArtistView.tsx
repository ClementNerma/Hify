import { useSuspenseQueries } from '#/api/hooks.ts'
import { fetchArtistAlbumParticipations, fetchArtistAlbums } from '#/api/queries.ts'
import { AlbumCard } from '#/components/molecules/AlbumCard.tsx'
import { NavRow } from '#/components/navigables/Row.tsx'

type ArtistViewProps = { artistId: string }

export function ArtistView({ artistId }: ArtistViewProps) {
  const [albumsByName, /*albumsByDate, albumsByScore,*/ albumParticipationsByName] =
    useSuspenseQueries(
      {
        queryKey: ['artistAlbums'],
        queryFn: () => fetchArtistAlbums(artistId, 'DATE', { dir: 'DESC', limit: 50, offset: 0 }),
      },
      {
        queryKey: ['artistALbumParticipations'],
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
