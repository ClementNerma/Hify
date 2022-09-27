import { derived, get, writable } from 'svelte/store'
import { AudioTrackFragment, AsyncPlayQueue } from '../graphql/generated'
import { readableAudioProgress, replayTrack, startAudioPlayer, stopAudioPlayer } from './audio-player'
import { logFatal, logInfo, logWarn } from './debugger'

type PlayQueue = {
  tracks: QueuedTrack[]
  position: number | null
}

type QueuedTrack = AudioTrackFragment & Readonly<{ idInQueue: string }>

const playQueue = writable<PlayQueue>({
  tracks: [],
  position: null,
})

export const readablePlayQueue = derived(playQueue, (_) => _)
export const currentTrack = derived(playQueue, ({ tracks, position }) => position !== null && tracks[position])
export const queuePosition = derived(playQueue, ({ position }) => position)

export const PREVIOUS_TRACK_OR_REWIND_THRESOLD_SECONDS = 5

function makeQueuedTrack(track: AudioTrackFragment): QueuedTrack {
  return { ...track, idInQueue: Math.random().toString() }
}

export async function playTrackFromFetchableQueue(tracksIds: string[], position: number): Promise<void> {
  if (!tracksIds[position]) {
    return logFatal('Provided track position does not exist in fetchable queue')
  }

  logInfo(`Fetching play queue for ${tracksIds.length} tracks...`)

  const tracks = await AsyncPlayQueue({ variables: { tracksIds } })

  logInfo(`Set new queue with ${tracks.data.selectTracks.length} tracks`)

  return playTrackFromNewQueue(tracks.data.selectTracks, position)
}

export async function playNewQueueFromBeginning(tracks: AudioTrackFragment[]): Promise<void> {
  playQueue.set({ tracks: tracks.map(makeQueuedTrack), position: 0 })
  startAudioPlayer(tracks[0], playNextTrack)
}

export async function playTrackFromNewQueue(tracks: AudioTrackFragment[], position: number): Promise<void> {
  playQueue.set({ tracks: tracks.map(makeQueuedTrack), position })
  startAudioPlayer(tracks[position], playNextTrack)
}

export function playTrackFromCurrentQueue(position: number): void {
  playQueue.update(({ tracks }) => {
    startAudioPlayer(tracks[position], playNextTrack)
    return { tracks, position }
  })
}

export function playPreviousTrackOrRewind(): void {
  logInfo('Going to play previous track or rewind...')

  const progress = get(readableAudioProgress)

  if (progress !== null && progress > PREVIOUS_TRACK_OR_REWIND_THRESOLD_SECONDS) {
    replayTrack()
  } else {
    playQueue.update(({ tracks, position }) => {
      let newPosition: number | null

      if (position === null) {
        newPosition = null
      } else if (position === 0) {
        replayTrack()
        newPosition = null
      } else {
        newPosition = position - 1
      }

      if (newPosition !== null) {
        startAudioPlayer(tracks[newPosition], playNextTrack)
        logInfo('Playing previous track at position: ' + newPosition.toString())
      } else {
        logInfo('No previous track to play')
      }

      return { tracks, position: newPosition }
    })
  }
}

export function playNextTrack(): void {
  logInfo('Going to play next track...')

  playQueue.update(({ tracks, position }) => {
    let newPosition: number | null

    if (position === null) {
      newPosition = tracks.length > 0 ? 0 : null
    } else if (position === tracks.length - 1) {
      stopAudioPlayer()
      newPosition = null
    } else {
      newPosition = position + 1
    }

    if (newPosition !== null) {
      startAudioPlayer(tracks[newPosition], playNextTrack)
      logInfo('Playing next track at position: ' + newPosition.toString())
    } else {
      logInfo('No more track to play')
    }

    return { tracks, position: newPosition }
  })
}

export function queueAsNext(list: AudioTrackFragment[]): void {
  logInfo(`Queuing ${list.length} track(s) as next`)

  playQueue.update(({ position, tracks }) => {
    return {
      position,
      tracks:
        position === null
          ? list.map(makeQueuedTrack)
          : tracks
              .slice(0, position + 1)
              .concat(list.map(makeQueuedTrack))
              .concat(tracks.slice(position + 1)),
    }
  })
}

export function removeFromQueue(index: number): void {
  logInfo(`Removing track n°${index + 1} from queue`)

  playQueue.update(({ position, tracks }) => {
    if (!Object.prototype.hasOwnProperty.call(tracks, index)) {
      logWarn('Cannot remove track from queue as the provided position does not exist')
      return { position, tracks }
    }

    if (index === position) {
      logWarn('Cannot remove track from queue as it is the currently-playing track')
      return { position, tracks }
    }

    return {
      position: position === null ? null : index < position ? position - 1 : position,
      tracks: tracks.slice(0, index).concat(tracks.slice(index + 1)),
    }
  })
}
