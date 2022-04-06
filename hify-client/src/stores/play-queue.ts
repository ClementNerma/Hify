import { derived, writable } from 'svelte/store'
import { AlbumYearStrategy, AudioTrackFragment, AsyncPlayQueue } from '../graphql/generated'
import { startAudioPlayer, stopAudioPlayer } from './audio-player'
import { logFatal, logInfo } from './debugger'

type PlayQueue = {
  tracks: AudioTrackFragment[]
  position: (() => number) | null
}

const playQueue = writable<PlayQueue>({
  tracks: [],
  position: null,
})

export const readablePlayQueue = derived(playQueue, (_) => _)
export const currentTrack = derived(playQueue, ({ tracks, position }) => position !== null && tracks[position])
export const queuePosition = derived(playQueue, ({ position }) => position)

export async function playTrackFromFetchableQueue(tracksIds: string[], position: number): Promise<void> {
  if (!tracksIds[position]) {
    return logFatal('Provided track position does not exist in fetchable queue')
  }

  logInfo(`Fetching play queue for ${tracksIds.length} tracks...`)

  const tracks = await AsyncPlayQueue({
    variables: {
      tracksIds,
      albumYearStrategy: AlbumYearStrategy.IdenticalOrFirstTrack,
    },
  })

  logInfo(`Set new queue with ${tracks.data.selectTracks.length} tracks`)

  return playTrackFromNewQueue(tracks.data.selectTracks, position)
}

export async function playTrackFromNewQueue(tracks: AudioTrackFragment[], position: number): Promise<void> {
  playQueue.set({ tracks, position })
  startAudioPlayer(tracks[position], playNextTrack)
}

export function playTrackFromCurrentQueue(position: number): void {
  playQueue.update(({ tracks }) => {
    startAudioPlayer(tracks[position], playNextTrack)
    return { tracks, position }
  })
}

export function playNextTrack(): void {
  logInfo('Going to play next track...')

  playQueue.update(({ tracks, position }) => {
    let newposition: (() => number) | null

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
