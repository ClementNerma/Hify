import { useRef, useState } from 'react'
import { FaSpinner } from 'react-icons/fa6'
import { searchAlbums, searchArtists, searchTracks } from '#/api/searches.ts'
import type {
  AlbumCompleteInfos,
  ArtistCompleteInfos,
  Paginated,
  TrackCompleteInfos,
} from '#/api/types.ts'
import { AlbumCard } from '#/components/molecules/AlbumCard.tsx'
import { ArtistCard } from '#/components/molecules/ArtistCard.tsx'
import { TrackCard } from '#/components/molecules/TrackCard.tsx'
import { BlockNavItem } from '#/components/navigables/Item.tsx'
import { NavRow } from '#/components/navigables/Row.tsx'
import { playTrackFromNewQueue } from '#/global/player.ts'
import { unwrapNotNull } from '#/utils/common.ts'
import { useValueWatcher } from '#/utils/hooks.ts'

export type SearchViewProps = { initialQuery?: string }

export function SearchView({ initialQuery }: SearchViewProps) {
  const inputDom = useRef<HTMLInputElement>(null)

  const [query, setQuery] = useState(initialQuery ?? '')

  const [searchResults, setSearchResults] = useState<SearchState | null>(null)
  const [isSearching, setIsSearching] = useState(false)

  const focusInput = () => {
    inputDom.current?.focus()
  }

  const unfocusInput = () => {
    inputDom.current?.blur()
  }

  const triggerSearch = async (query: string) => {
    setIsSearching(true)

    // TODO: pagination
    const [tracks, albums, artists] = await Promise.all([
      searchTracks(query, { dir: 'ASC', limit: 50, offset: null }),
      searchAlbums(query, { dir: 'ASC', limit: 50, offset: null }),
      searchArtists(query, { dir: 'ASC', limit: 50, offset: null }),
    ])

    setSearchResults({ tracks, albums, artists })
    setIsSearching(false)
  }

  useValueWatcher(
    initialQuery,
    (query) => {
      if (query !== undefined && query !== '') {
        // oxlint-disable-next-line typescript/no-floating-promises
        triggerSearch(query)
      }
    },
    { immediate: true },
  )

  const [inputInterval, setInputInterval] = useState<number | null>(null)

  const onInput = () => {
    const query = unwrapNotNull(inputDom.current).value

    setQuery(query)
    history.replaceState(null, '', `/search/${encodeURIComponent(query)}`)

    if (inputInterval !== null) {
      clearTimeout(inputInterval)
    }

    if (query.length < 3) {
      return
    }

    const interval = setTimeout(() => {
      // oxlint-disable-next-line typescript/no-floating-promises
      triggerSearch(query)
    }, 500)

    setInputInterval(interval)
  }

  return (
    <>
      <div className="flex justify-center mt-2">
        <BlockNavItem className="w-1/4" onFocused={focusInput} onUnfocused={unfocusInput}>
          <div className="relative">
            <input
              ref={inputDom}
              type="text"
              className="p-1 border border-gray-600 w-full outline-none"
              onInput={onInput}
              value={query}
            />

            {isSearching && (
              <div className="absolute inset-y-0 right-0 flex items-center pr-3">
                <FaSpinner className="animate-spin" />
              </div>
            )}
          </div>
        </BlockNavItem>
      </div>

      {searchResults && (
        <div className="mt-4 px-4 space-y-6">
          <div>
            <h2>Tracks</h2>

            {searchResults.tracks.results.length === 0 ? (
              <p>
                <em>No track found.</em>
              </p>
            ) : (
              <NavRow className="items-row-auto">
                {searchResults.tracks.results.map((track, i) => (
                  <TrackCard
                    key={track.track.id}
                    track={track}
                    onPress={() =>
                      playTrackFromNewQueue(searchResults.tracks.results, i, {
                        gotoPlayer: true,
                        fromMix: null,
                      })
                    }
                  />
                ))}
              </NavRow>
            )}

            <h2>Albums</h2>

            {searchResults.albums.results.length === 0 ? (
              <p>
                <em>No album found.</em>
              </p>
            ) : (
              <NavRow className="items-row-auto">
                {searchResults.albums.results.map((album) => (
                  <AlbumCard album={album} key={album.album.id} />
                ))}
              </NavRow>
            )}

            <h2>Artists</h2>

            {searchResults.artists.results.length === 0 ? (
              <p>
                <em>No artist found.</em>
              </p>
            ) : (
              <NavRow className="items-row-auto">
                {searchResults.artists.results.map(({ artist }) => (
                  <ArtistCard artist={artist} key={artist.id} />
                ))}
              </NavRow>
            )}
          </div>
        </div>
      )}
    </>
  )
}

type SearchState = {
  tracks: Paginated<TrackCompleteInfos>
  albums: Paginated<AlbumCompleteInfos>
  artists: Paginated<ArtistCompleteInfos>
}
