import { derived, writable } from 'svelte/store'
import { AlbumYearStrategy, AudioTrackFragment, AsyncPlayQueue } from '../graphql/generated'
import { startAudioPlayer } from './audio-player'
import { logFatal, logInfo } from './debugger'

type PlayQueue = {
  tracks: AudioTrackFragment[]
  position: number | null
}

const playQueue = writable<PlayQueue>({
  tracks: [],
  position: null,
})

export const readablePlayQueue = derived(playQueue, (_) => _)
export const currentTrack = derived(playQueue, ({ tracks, position }) => position !== null && tracks[position])

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

  return playTrackFromQueue(tracks.data.selectTracks, position)
}

export async function playTrackFromQueue(tracks: AudioTrackFragment[], position: number): Promise<void> {
  playQueue.set({ tracks, position })
  startAudioPlayer(tracks[position])
}