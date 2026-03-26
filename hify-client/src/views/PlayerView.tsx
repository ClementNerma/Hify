import { useState } from 'react'
import {
  FaCirclePause,
  FaCompactDisc,
  FaItunesNote,
  FaMicrophoneLines,
  FaPause,
  FaStop,
  FaTriangleExclamation,
  FaXmark,
} from 'react-icons/fa6'
import type { TrackCompleteInfos } from '#/api/types.ts'
import { urls } from '#/api/urls.ts'
import { DfFadeOut, DfShow } from '#/components/molecules/DfToggle.tsx'
import { EditableRating } from '#/components/molecules/EditableRating.tsx'
import { OneLineList } from '#/components/molecules/OneLineList.tsx'
import { TrackCard } from '#/components/molecules/TrackCard.tsx'
import { BlockNavItem, NavItem } from '#/components/navigables/Item.tsx'
import { NavRow } from '#/components/navigables/Row.tsx'
import type { ContextMenuItem } from '#/components/organisms/ContextMenu.tsx'
import { isBlackOutModeFeatureEnabledStore } from '#/global/bo-mode.ts'
import { defaultCtxMenus } from '#/global/ctx-menu.tsx'
import { isDfModeActiveStore, useDistractionFree } from '#/global/df-mode.ts'
import { navigationManager } from '#/global/nav.ts'
import {
  audioProgressStore,
  audioStateStore,
  playerStateStore,
  playTrackFromCurrentQueue,
  removeTrackFromQueue,
  seekAudio,
  toggleAudioPlaying,
} from '#/global/player.ts'
import { navigate } from '#/router/routes.ts'
import { routes } from '#/routes.ts'
import { formatDuration } from '#/utils/common.ts'
import { useResettableTimeout, useValuesWatcher, useValueWatcher } from '#/utils/hooks.ts'
import { useGlobalStore } from '#/utils/stores.ts'

export function PlayerView() {
  const { currentTrack, playQueue } = useGlobalStore(playerStateStore)

  const dfModeActive = useDistractionFree(currentTrack !== null)

  if (currentTrack === null) {
    return <h1 className="fixed top-1/3 w-full text-center">Queue is empty</h1>
  }

  const {
    album: { album },
  } = playQueue[currentTrack]

  return (
    <>
      {/* Background album art */}
      <div
        className="fixed inset-0 -z-20 bg-center bg-no-repeat bg-cover"
        style={{ backgroundImage: `url("${urls.albumArt(album, 'large')}")` }}
      />

      {/* Background blurring to increase readibility above background (album art) */}
      <div
        className="fixed inset-0 -z-10"
        style={{ backdropFilter: `blur(20px) brightness(${dfModeActive ? 0.4 : 0.3})` }}
      />

      {/* Album art (front) */}
      <img
        src={urls.albumArt(album, 'large')}
        className={`fixed top-[10%] left-[10%] w-4/5 h-4/5 m-auto overflow-auto object-contain transition-opacity duration-300 drop-shadow-[0_0_1em_rgb(55,55,55)] ${
          dfModeActive ? 'opacity-100' : 'opacity-50'
        }`}
      />

      {/* Bottom panel (current track infos + play queue) */}
      <DfFadeOut>
        <PlayerBottomPanel />
      </DfFadeOut>

      {/* Black out filter (when enabled, darkens the screen in DF mode) */}
      <BlackOutFilter />

      {/* Pause indicator when black out mode is active, to remind the TV is still on after a long period of inactivity */}
      <BlackOutPauseIndicator />

      {/* New track notification (in DF mode) */}
      <DfShow>
        <NewTrackNotification />
      </DfShow>
    </>
  )
}

// Bottom panel showing current track infos and play queue
function PlayerBottomPanel() {
  const { currentTrack, playQueue } = useGlobalStore(playerStateStore)
  const progress = useGlobalStore(audioProgressStore)

  return (
    <div className="fixed inset-x-0 -bottom-25 [&:has([class*='items-row-']_[data-navigable-focused])]:bottom-0 [&:has([class*='items-row-']:hover)]:bottom-0 px-[5%] pb-[1%] bg-linear-to-b from-red-500/0 to-[#1e1e1e] transition-[bottom] duration-300">
      {currentTrack !== null && (
        <>
          <TrackInfos track={playQueue[currentTrack]} />

          <BlockNavItem
            className="border-none accent-amber-500 nav-focused:accent-cyan-600"
            onPress={() => toggleAudioPlaying()}
            onLeftKey={() => seekAudio(-30)}
            onRightKey={() => seekAudio(30)}
          >
            <input
              type="range"
              min="0"
              max={playQueue[currentTrack].track.metadata.durationS}
              value={progress?.seconds ?? 0}
              readOnly
              className="w-full"
            />
          </BlockNavItem>
        </>
      )}

      <NavRow
        className="items-row-auto nav-unfocused:opacity-20 hover:opacity-100! transition-opacity duration-300"
        focusChildOnEnter={currentTrack}
        fixedNavId={PLAY_QUEUE_NAV_ID}
      >
        {playQueue.map((track, i) => (
          <TrackCard
            key={`${i}:${track.track.id}`}
            track={track}
            className={currentTrack === i ? 'bg-amber-900' : 'nav-unfocused:opacity-50'}
            onPress={() => playTrackFromCurrentQueue(i)}
            replaceCtxMenu={trackMenuItems(track, i)}
          />
        ))}
      </NavRow>
    </div>
  )
}

// Global navigation ID for the play queue (to focus items inside it)
//
// TODO: remove once issue about focus for left tracks has been fixed
const PLAY_QUEUE_NAV_ID = 'nav-player-play-queue'

// Display track informations on a single line
function TrackInfos({ track }: { track: TrackCompleteInfos }) {
  const progress = useGlobalStore(audioProgressStore)
  const { album, artists } = track
  const { metadata, tags } = track.track

  return (
    <NavRow className="align-top items-center *:p-1 *:line-clamp-1">
      <NavItem
        onPress={() =>
          navigate(routes.search, {
            initialQuery: tags.title,
          })
        }
      >
        <FaItunesNote /> {tags.title}
      </NavItem>

      <NavItem
        onPress={() => {
          navigate(routes.album, { albumId: album.album.id })
        }}
      >
        <FaCompactDisc /> {album.album.name}
      </NavItem>

      {artists.length > 0 && (
        <OneLineList
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
      )}

      <EditableRating
        track={track}
        onUpdated={(rating) => {
          // NOTE: good enough as current page (and player queue) will only use this reference,
          //       and any page change clears the queries cache to fetch the updated data
          //
          // oxlint-disable-next-line react-hooks-js/immutability
          track.rating = rating
        }}
      />

      <span className="ml-auto">
        <PlayerStatusIndicator /> {progress !== null ? formatDuration(progress.seconds) : '--:--'} /{' '}
        {formatDuration(metadata.durationS)}
      </span>
    </NavRow>
  )
}

// Icon to show the player status (playing, paused, stopped, error)
function PlayerStatusIndicator() {
  const audioState = useGlobalStore(audioStateStore)

  switch (audioState) {
    case 'playing': {
      // TODO: replace custom 'animation-duration' when Tailwind supports it natively
      return <FaCompactDisc className="animate-spin [animation-duration:2000ms]" />
    }

    case 'paused': {
      return <FaCirclePause />
    }

    case 'stopped': {
      return <FaStop />
    }

    case 'error': {
      return <FaTriangleExclamation />
    }

    default: {
      return null
    }
  }
}

// Darkens the screen when Black Out mode is enabled and DF mode is active
//
// Shows an opaque <div> on top of the screen, with `pointer-events: none` so it doesn't interfere
// with user interactions (notably `mouseenter` and `mouseleave`).
//
// When the component mounts, it triggers a fade-in transition to blacken the screen over a few seconds,
// and when unmounting it fades out the black overlay.
//
// A delay is applied to the fade-in transition so that the screen doesn't black out immediately when DF mode
// is activated, giving the user some time to interact with the UI before the screen darkens.
function BlackOutFilter() {
  const isBlackOutModeEnabled = useGlobalStore(isBlackOutModeFeatureEnabledStore)
  const dfModeActive = useGlobalStore(isDfModeActiveStore)

  const enabled = isBlackOutModeEnabled && dfModeActive

  return (
    <div
      className={`fixed inset-0 bg-black opacity-0 transition duration-1000 z-10 pointer-events-none ${enabled ? 'opacity-100 duration-[3s] ease-in delay-1000' : ''}`}
    />
  )
}

function BlackOutPauseIndicator() {
  const isBlackOutModeEnabled = useGlobalStore(isBlackOutModeFeatureEnabledStore)
  const dfModeActive = useGlobalStore(isDfModeActiveStore)
  const audioState = useGlobalStore(audioStateStore)

  const [visible, setVisible] = useState(false)

  const timeout = useResettableTimeout(() => setVisible(true), 5 * 60 * 1000) // 5 minutes

  useValuesWatcher(
    [isBlackOutModeEnabled, dfModeActive, audioState],
    () => {
      if (
        isBlackOutModeEnabled &&
        dfModeActive &&
        (audioState === 'paused' || audioState === 'stopped')
      ) {
        timeout.start()
      } else {
        setVisible(false)
        timeout.clear()
      }
    },
    { immediate: true },
  )

  return (
    visible && (
      <div className="fixed top-5 right-5 z-90">
        {audioState === 'paused' && <FaPause className=" text-4xl" />}
        {audioState === 'stopped' && <FaStop className=" text-4xl" />}
      </div>
    )
  )
}

function NewTrackNotification() {
  // Track to show the notification for. If set to `null`, the notification is hidden
  const [showForTrack, setShowForTrack] = useState<TrackCompleteInfos | null>(null)

  // Timeout to hide the notification
  const timeout = useResettableTimeout(() => setShowForTrack(null), 3000)

  // React to track changes to show the notification
  const { currentTrack, playQueue } = useGlobalStore(playerStateStore)

  useValueWatcher(currentTrack, () => {
    if (currentTrack !== null) {
      // Show the notification
      setShowForTrack(playQueue[currentTrack])

      // Hide it after a delay (restart the timer if already running)
      timeout.restart()
    } else {
      timeout.clear({ runCallback: true })
    }
  })

  if (showForTrack === null) {
    return null
  }

  const { track, album } = showForTrack

  return (
    <div className="fixed top-4 left-4 bg-gray-700 rounded-lg shadow-lg p-3 flex items-center max-w-sm z-50">
      {/* <!-- Icon --> */}
      <div className="w-12 h-12 rounded-md flex items-center shrink-0">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className="h-8 w-8 text-white"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="2"
            d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"
          />
        </svg>
      </div>

      {/* <!-- Track Info --> */}
      <div className="flex flex-col overflow-hidden">
        <span className="text-white font-semibold truncate">{track.tags.title}</span>
        <span className="text-gray-300 text-sm truncate">{album.album.name}</span>
      </div>
    </div>
  )
}

function trackMenuItems(track: TrackCompleteInfos, position: number): ContextMenuItem[] {
  return [
    ...defaultCtxMenus.track(track),

    {
      icon: <FaXmark />,
      label: 'Remove from queue',
      onPress() {
        const { playQueue } = playerStateStore.getSnapshot()

        if (position - 1 >= 0) {
          navigationManager.focusChildOf(PLAY_QUEUE_NAV_ID, position - 1, 'RIGHT')
        } else if (position + 1 < playQueue.length) {
          // TODO: doesn't work reliably as the ID of each item changes when the queue is updated (see `key` property on the play queue's `NavItems`)
          // so -> find another way!
          //
          // One idea: determine the target ID (`${position}:${navId}`) and then *schedule* a focus on it after the queue has been updated using the navigationManager
          // Another idea: store that ID somewhere and put an onReady() handler on the TrackCards to check if it matches the stored ID, and focus itself if so
          navigationManager.focusChildOf(PLAY_QUEUE_NAV_ID, position + 1, 'LEFT')
        }

        removeTrackFromQueue(position)
      },
      skipFocusRestore: true,
    },
  ]
}
