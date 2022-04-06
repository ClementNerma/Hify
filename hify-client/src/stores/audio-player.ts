import { derived, get, writable } from 'svelte/store'
import { getStreamUri } from '../rest-api'
import { logInfo, logDebug, logWarn, logError } from './debugger'
import { AudioTrackFragment } from '../graphql/generated'

const audioPlayer = writable<HTMLAudioElement | null>(null)
const audioProgress = writable<number | null>(null)
const audioPaused = writable<boolean | null>(null)

export const readableAudioProgress = derived(audioProgress, (_) => _)
export const readableAudioPaused = derived(audioPaused, (_) => _)

export function startAudioPlayer(track: AudioTrackFragment, nextHandler: () => void, play = true) {
  audioPlayer.update((prev): HTMLAudioElement => {
    if (prev && !prev.paused) {
      prev.pause()
    }

    logInfo(`Started playing track with ID: ${track.id} | ${track.metadata.tags.title}`)

    let lastTimeUpdate = 0

    const newAudio = new Audio(getStreamUri(track.id))

    newAudio.addEventListener('error', (e) => logError('Failed to load audio track', e))
    newAudio.addEventListener('play', () => audioPaused.set(false))
    newAudio.addEventListener('pause', () => audioPaused.set(true))
    newAudio.addEventListener('ended', nextHandler)
    newAudio.addEventListener('timeupdate', () => {
      const currentTime = Math.round(newAudio.currentTime)

      if (currentTime !== lastTimeUpdate) {
        lastTimeUpdate = currentTime
        audioProgress.set(currentTime)
      }
    })

    if (play !== false) {
      newAudio
        .play()
        .then(() => {
          audioPaused.set(false)
          audioProgress.set(0)
        })
        .catch((e: unknown) => logError('Failed to play audio', e))
    }

    return newAudio
  })
}

export function setPlayingAudioProgress(seconds: number) {
  const player = get(audioPlayer)

  if (!player) {
    logWarn('Tried to set audio progress while no audio is playing')
    return
  }

  player.currentTime = seconds

  logDebug(`Set relative audio progress: ${humanReadableDuration(seconds)}`)
}

export function setPlayingAudioProgressRelative(relativeSeconds: number) {
  const player = get(audioPlayer)

  if (!player) {
    logWarn('Tried to set audio progress while no audio is playing')
    return
  }

  player.currentTime += relativeSeconds

  logDebug(`Set relative audio progress: ${relativeSeconds}s`)
}

export function toggleAudioPlayback() {
  const player = get(audioPlayer)

  if (!player) {
    logWarn('Tried to toggle audio playback while no audio is playing')
    return
  }

  logInfo('Toggled audio playback')

  if (player.paused) {
    player.play().catch((e) => alert('Failed to resume audio: ' + (e instanceof Error ? e.message : '<unknown error>')))
  } else {
    player.pause()
  }
}

export function stopAudioPlayer() {
  const player = get(audioPlayer)

  if (!player) {
    logWarn('Tried to stop audio playback while no audio is playing')
    return
  }

  logInfo('Stopped audio playback')

  if (!player.paused) {
    player.pause()
  }

  player.currentTime = 0
}

export function humanReadableDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  seconds -= hours * 3600

  const minutes = Math.floor(seconds / 60)
  seconds -= minutes * 60

  const hoursPrefix = hours > 0 ? `${hours < 10 ? '0' : ''}${hours}:` : ''

  return `${hoursPrefix}${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`
}
