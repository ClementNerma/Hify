import { derived, get, writable } from 'svelte/store'
import { AlbumYearStrategy } from '../../graphql/types'
import { getStreamUri } from '../../rest-api'
import { AsyncAudioTrack, AudioTrackQuery } from './AudioTrack.generated'
import { logInfo, logDebug, logWarn } from '../debugger'

type AudioPlayingState = {
  htmlEl: HTMLAudioElement
  trackId: string
  trackInfos: Promise<AudioTrackQuery['track']>
}

const audioPlaying = writable<AudioPlayingState | null>(null)
const audioProgress = writable<number | null>(null)
const audioPaused = writable<boolean | null>(null)

audioPlaying.subscribe((playing) => {
  if (!playing) {
    audioProgress.set(null)
  }
})

export const readableAudioPlaying = derived(audioPlaying, (_) => _)
export const readableAudioProgress = derived(audioProgress, (_) => _)
export const readableAudioPaused = derived(audioPaused, (_) => _)

export function playTrack(trackId: string, play = true) {
  audioPlaying.update((prevAudio): AudioPlayingState => {
    if (prevAudio && !prevAudio.htmlEl.paused) {
      prevAudio.htmlEl.pause()
      audioPaused.set(true)
    }

    logInfo('Started playing track ID: ' + trackId)

    const newAudio = new Audio(getStreamUri(trackId))

    newAudio.addEventListener('error', (e) =>
      alert('Failed to load audio track: ' + (e instanceof Error ? e.message : '<unknown error>')),
    )

    if (play !== false) {
      newAudio
        .play()
        .then(() => {
          audioPaused.set(false)
          audioProgress.set(0)
        })
        .catch((e) => alert('Failed to play audio: ' + (e instanceof Error ? e.message : '<unknown error>')))
    }

    let lastTimeUpdate = 0

    newAudio.addEventListener('timeupdate', () => {
      const currentTime = Math.round(newAudio.currentTime)

      if (currentTime !== lastTimeUpdate) {
        lastTimeUpdate = currentTime
        audioProgress.set(currentTime)
      }
    })

    return {
      htmlEl: newAudio,
      trackId,
      trackInfos: AsyncAudioTrack({
        variables: {
          trackId,
          albumYearStrategy: AlbumYearStrategy.IdenticalOrFirstTrack,
        },
      }).then((res) => res.data.track),
    }
  })
}

export function setPlayingAudioProgress(seconds: number) {
  const playing = get(audioPlaying)

  if (!playing) {
    logWarn('Tried to set audio progress while no audio is playing')
    return
  }

  playing.htmlEl.currentTime = seconds

  logDebug(`Set audio progress: ${humanReadableDuration(seconds)}s`)
}

export function setPlayingAudioProgressRelative(relativeSeconds: number) {
  const playing = get(audioPlaying)

  if (!playing) {
    logWarn('Tried to set audio progress while no audio is playing')
    return
  }

  playing.htmlEl.currentTime += relativeSeconds

  logDebug(`Set relative audio progress: ${relativeSeconds}s`)
}

export function toggleAudioPlayback() {
  const playing = get(audioPlaying)

  if (!playing) {
    logWarn('Tried to toggle audio playback while no audio is playing')
    return
  }

  logInfo('Toggled audio playback')

  if (playing.htmlEl.paused) {
    playing.htmlEl
      .play()
      .catch((e) => alert('Failed to resume audio: ' + (e instanceof Error ? e.message : '<unknown error>')))
    audioPaused.set(false)
  } else {
    playing.htmlEl.pause()
    audioPaused.set(true)
  }
}

export function humanReadableDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  seconds -= hours * 3600

  const minutes = Math.floor(seconds / 60)
  seconds -= minutes * 60

  const hoursPrefix = hours > 0 ? `${hours < 10 ? '0' : ''}${hours}:` : ''

  return `${hoursPrefix}${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`
}
