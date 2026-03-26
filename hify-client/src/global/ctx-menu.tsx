import { FaCompactDisc, FaMicrophoneLines, FaTurnDown } from 'react-icons/fa6'
import { fetchAlbumTracks } from '#/api/queries.ts'
import type { AlbumCompleteInfos, TrackCompleteInfos } from '#/api/types.ts'
import type { ContextMenuItem, ContextMenuStatus } from '#/components/organisms/ContextMenu.tsx'
import { navigate } from '#/router/routes.ts'
import { routes } from '#/routes.ts'
import { randomId } from '#/utils/common.ts'
import { createGlobalStore } from '#/utils/stores.ts'
import { enqueueTracksAsNext } from './player'

export const ctxMenuStatusStore = createGlobalStore<ContextMenuStatus>({ type: 'closed' })

export function openContextMenu(items: ContextMenuItem[]): void {
  ctxMenuStatusStore.mutate({
    type: 'opened',
    items: items.map((item) => ({ ...item, id: randomId() })),
  })
}

export function closeContextMenu(): void {
  ctxMenuStatusStore.mutate({ type: 'closed' })
}

export const defaultCtxMenus = {
  //
  // => Track
  //
  track: (track: TrackCompleteInfos): ContextMenuItem[] =>
    [
      {
        icon: <FaTurnDown className="rotate-270" />,
        label: 'Play next',
        onPress: () => enqueueTracksAsNext([track]),
      },
      {
        icon: <FaCompactDisc />,
        label: `"${track.album.album.name}"`,
        onPress: () => {
          navigate(routes.album, { albumId: track.album.album.id })
        },
        skipFocusRestore: true,
      },
      ...track.artists.map(
        ({ artist }): ContextMenuItem => ({
          icon: <FaMicrophoneLines />,
          label: `"${artist.name}"`,
          onPress: () => {
            navigate(routes.artist, { artistId: artist.id })
          },
          skipFocusRestore: true,
        }),
      ),
    ] satisfies ContextMenuItem[],

  //
  // => Albums
  //
  album: ({ album, artists }: AlbumCompleteInfos): ContextMenuItem[] => [
    {
      icon: <FaTurnDown className="rotate-270" />,
      label: 'Play next',
      onPress: () => {
        // oxlint-disable-next-line typescript/no-floating-promises
        fetchAlbumTracks(album.id).then(enqueueTracksAsNext)
      },
    },
    {
      icon: <FaTurnDown className="rotate-270" />,
      label: 'Enqueue great tracks',
      onPress: () => {
        // oxlint-disable-next-line typescript/no-floating-promises
        fetchAlbumTracks(album.id).then((tracks) =>
          enqueueTracksAsNext(
            tracks.filter((track) => track.rating === 'Four' || track.rating === 'Five'),
          ),
        )
      },
    },
    {
      icon: <FaTurnDown className="rotate-270" />,
      label: 'Enqueue random great tracks',
      onPress: () => {
        // oxlint-disable-next-line typescript/no-floating-promises
        fetchAlbumTracks(album.id).then((tracks) =>
          enqueueTracksAsNext(
            tracks
              .filter((track) => track.rating === 'Four' || track.rating === 'Five')
              .toSorted(() => Math.random() - 0.5),
          ),
        )
      },
    },
    ...artists.map(
      ({ artist }): ContextMenuItem => ({
        icon: <FaMicrophoneLines />,
        label: `"${artist.name}"`,
        onPress: () => {
          navigate(routes.artist, { artistId: artist.id })
        },
        skipFocusRestore: true,
      }),
    ),
  ],
}
