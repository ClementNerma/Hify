import { fetchMultiTracks, mixTracks } from '#/api/queries.ts'
import type { Pagination, TrackCompleteInfos, UserMixParams } from '#/api/types.ts'
import { urls } from '#/api/urls.ts'
import { navigate } from '#/router/routes.ts'
import { routes } from '#/routes.ts'
import { arrayWithInsertion, arrayWithoutIndex, assertNotNull } from '#/utils/common.ts'
import { createGlobalStore } from '#/utils/stores.ts'
import { showFailure, showNotification, showQuickNotification } from './notifications'
import { loadPersistentData, prependHistoryTrack, updatePersistedPlayerState } from './persistent'

// TODO: don't show error notification for audio player's AbortError, which can happen when quickly skipping tracks or when navigating away from the player view while a track is loading

//
// => Player state
//

export type PlayerState = {
  currentMix: PlayerMix | null
  playQueue: TrackCompleteInfos[]
  currentTrack: number | null
}

type PlayerMix = { params: UserMixParams; consumedTracks: number; hasMore: boolean }

let audioPlayer: HTMLAudioElement | null = null

// TODO: don't export this and export a `usePlayerState` hook instead
// TODO: do the same for `audioStateStore` and `audioProgressStore`
export const playerStateStore = createGlobalStore<PlayerState>({
  currentMix: null,
  playQueue: [],
  currentTrack: null,
})

playerStateStore.subscribe(({ currentMix, playQueue, currentTrack }) => {
  updatePersistedPlayerState({
    currentMix,
    queueTrackIds: playQueue.map((track) => track.track.id),
    currentTrack,
  })
})

//
// => Audio state
//

export type AudioState = 'playing' | 'paused' | 'stopped' | 'error'

export const audioStateStore = createGlobalStore<AudioState>('stopped')

//
// => Audio progress
//

export const audioProgressStore = createGlobalStore<{
  forTrackId: string
  seconds: number
} | null>(null)

//
// => Functions
//

export function playTrackFromNewQueue(
  tracks: TrackCompleteInfos[],
  position: number,
  opts: { gotoPlayer: boolean; fromMix: PlayerMix | null },
): void {
  if (Math.floor(position) !== position || position < 0 || position >= tracks.length) {
    showFailure('Cannot play track: position out of bounds in provided queue')
    return
  }

  if (audioPlayer) {
    if (!audioPlayer.paused) {
      // Don't keep playing the previous track while loading the new one
      audioPlayer.pause()
    }

    // Avoid showing a non-reset progress bar when going to the player view
    audioPlayer.currentTime = 0
  }

  // Fetch next tracks from mix
  if (position >= tracks.length - 5 && opts.fromMix?.hasMore === true) {
    const { consumedTracks, params } = opts.fromMix
    const newPagination: Pagination = { offset: consumedTracks, limit: 50, dir: 'ASC' }

    // oxlint-disable-next-line typescript/no-floating-promises
    mixTracks(params, newPagination).then((newTracks) => {
      playerStateStore.mutateWith((prev) => ({
        ...prev,
        currentMix: {
          params,
          consumedTracks: consumedTracks + newTracks.results.length,
          hasMore: newTracks.hasMore,
        },
        playQueue: [...prev.playQueue, ...newTracks.results],
      }))
    })
  }

  const track = tracks[position]

  playerStateStore.mutateWith((prev) => ({
    ...prev,
    currentMix: opts.fromMix ?? null,
    playQueue: tracks,
    currentTrack: position,
  }))

  // Only navigating after the audio starts ensure we don't see flickering
  // with the previous play queue and album art
  if (opts.gotoPlayer) {
    navigate(routes.player, {})
  }

  audioProgressStore.mutate({ forTrackId: track.track.id, seconds: 0 })

  const audio = new Audio(urls.trackAudioUrl(track.track.id))
  audioPlayer = audio

  audio.addEventListener('error', () => audioStateStore.mutate('error'))
  audio.addEventListener('play', () => audioStateStore.mutate('playing'))
  audio.addEventListener('pause', () => audioStateStore.mutate('paused'))

  audio.addEventListener('ended', () => playNextTrack())

  let previousSeconds = 0
  let listeningDuration = 0
  let addedToHistory = false

  audio.addEventListener('timeupdate', () => {
    const seconds = Math.floor(audio.currentTime)

    if (
      // Don't increase listening duration in case of jump
      seconds < previousSeconds + 3 &&
      // Nor when going back
      seconds > previousSeconds
    ) {
      listeningDuration += seconds - previousSeconds

      if (listeningDuration >= 10 && !addedToHistory) {
        addedToHistory = true
        prependHistoryTrack(track)
      }
    }

    if (seconds !== previousSeconds) {
      previousSeconds = seconds
      audioProgressStore.mutate({ forTrackId: track.track.id, seconds })
    }
  })

  audio.play().catch((e: unknown) => {
    audioStateStore.mutate('error')
    showNotification({ type: 'error', title: 'Failed to play', message: String(e) })
  })
}

export function playTrackFromCurrentQueue(position: number): void {
  const { playQueue, currentMix } = playerStateStore.getSnapshot()

  playTrackFromNewQueue(playQueue, position, { gotoPlayer: false, fromMix: currentMix })
}

export function toggleAudioPlaying(): void {
  const audioState = audioStateStore.getSnapshot()

  if (!audioPlayer) {
    if (audioState === 'stopped' || audioState === 'error') {
      const { playQueue, currentTrack } = playerStateStore.getSnapshot()

      if (playQueue.length > 0) {
        playTrackFromCurrentQueue(currentTrack ?? 0)
      }

      navigate(routes.player, {})
    }

    return
  }

  if (audioState === 'playing') {
    audioPlayer.pause()
  } else {
    audioPlayer.play().catch((e: unknown) => {
      audioStateStore.mutate('error')
      showNotification({ type: 'error', title: 'Failed to resume playing', message: String(e) })
    })
  }
}

export function pauseAudio(): void {
  if (audioPlayer && !audioPlayer.paused) {
    audioPlayer.pause()
  }
}

export function playNextTrack(): void {
  const { currentTrack, playQueue, currentMix } = playerStateStore.getSnapshot()

  if (currentTrack === null) {
    if (playQueue.length > 0) {
      playTrackFromCurrentQueue(0)
    }

    return
  }

  const nextPosition = currentTrack + 1

  if (nextPosition < playQueue.length) {
    playTrackFromCurrentQueue(nextPosition)
  } else if (currentMix?.hasMore === true) {
    showFailure('Unexpected: tracks should have been loaded ahead of time')
  } else {
    audioPlayer?.pause()

    showQuickNotification({
      type: 'info',
      title: 'End of queue',
      message: 'No more tracks to play',
    })
  }
}

export function rewindorPlayPrevTrack(): void {
  const { currentTrack } = playerStateStore.getSnapshot()

  if (currentTrack === null) {
    return
  }

  assertNotNull(audioPlayer)

  if (audioPlayer.currentTime > 5) {
    audioPlayer.currentTime = 0
    return
  }

  const prevPosition = currentTrack - 1

  if (prevPosition >= 0) {
    playTrackFromCurrentQueue(prevPosition)
  } else {
    audioPlayer.currentTime = 0
  }
}

export function seekAudio(relativeSeconds: number): void {
  if (!audioPlayer) {
    return
  }

  let newTime = audioPlayer.currentTime + relativeSeconds

  if (newTime < 0) {
    newTime = 0
  } else if (newTime > audioPlayer.duration) {
    newTime = audioPlayer.duration
  }

  audioPlayer.currentTime = newTime
}

export function enqueueTracksAsNext(tracks: TrackCompleteInfos[]): void {
  playerStateStore.mutateWith(({ currentMix, currentTrack, playQueue }) => {
    if (playQueue.length === 0) {
      return { currentMix: null, playQueue: tracks, currentTrack: 0 }
    }

    assertNotNull(currentTrack)

    return {
      currentMix,
      currentTrack,
      playQueue: arrayWithInsertion(playQueue, currentTrack + 1, tracks),
    }
  })

  showQuickNotification({
    type: 'info',
    title: 'Queue updated',
    message: `${tracks.length} ${tracks.length > 1 ? 'tracks have' : 'track has'} been added to queue`,
  })
}

export function removeTrackFromQueue(position: number): void {
  const { playQueue, currentTrack } = playerStateStore.getSnapshot()

  if (position < 0 || position >= playQueue.length) {
    showFailure('Cannot remove track from queue: position out of bounds')
    return
  }

  if (currentTrack === null) {
    playerStateStore.mutateWith((prev) => ({
      ...prev,
      playQueue: arrayWithoutIndex(prev.playQueue, position),
    }))

    return
  }

  if (playQueue.length === 1) {
    playerStateStore.mutateWith((prev) => ({
      ...prev,
      playQueue: [],
      currentTrack: null,
    }))

    return
  }

  if (position < currentTrack) {
    // Adjust current track index
    playerStateStore.mutateWith((prev) => ({
      ...prev,
      currentTrack: prev.currentTrack !== null ? prev.currentTrack - 1 : null,
      playQueue: arrayWithoutIndex(prev.playQueue, position),
    }))
  } else if (position > currentTrack) {
    playerStateStore.mutateWith((prev) => ({
      ...prev,
      playQueue: arrayWithoutIndex(prev.playQueue, position),
    }))
  } else {
    showFailure('Cannot remove currently playing track from queue')
  }
}

export function playNewMix(mixParams: UserMixParams): void {
  // oxlint-disable-next-line typescript/no-floating-promises
  mixTracks(mixParams, { limit: 100, offset: null }).then((tracks) => {
    playTrackFromNewQueue(tracks.results, 0, {
      gotoPlayer: true,
      fromMix: {
        params: mixParams,
        consumedTracks: tracks.results.length,
        hasMore: tracks.hasMore,
      },
    })
  })
}

export async function loadPersistedPlayerState(): Promise<void> {
  const { playerState } = loadPersistentData()

  if (!playerState || playerState.queueTrackIds.length === 0) {
    return
  }

  const { currentMix, currentTrack, queueTrackIds } = playerState

  const playQueue = await fetchMultiTracks(queueTrackIds)

  playerStateStore.mutate({
    currentMix,
    playQueue,
    currentTrack,
  })
}
