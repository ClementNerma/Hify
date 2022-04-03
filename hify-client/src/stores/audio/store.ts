import { derived, get, writable } from 'svelte/store'
import { AlbumYearStrategy } from '../../graphql/types'
import { getStreamUri } from '../../rest-api'
import { AsyncAudioTrack, AudioTrackQuery } from './AudioTrack.generated'

type AudioPlayingState = {
  htmlEl: HTMLAudioElement
  trackId: string
  trackInfos: Promise<AudioTrackQuery['track']>
}

const audioPlaying = writable<AudioPlayingState | null>(null)
const audioProgress = writable<number | null>(null)

audioPlaying.subscribe((playing) => {
  if (!playing) {
    audioProgress.set(null)
  }
})

export const readableAudioPlaying = derived(audioPlaying, (_) => _)
export const readableAudioProgress = derived(audioProgress, (_) => _)

export function playTrack(trackId: string) {
  audioPlaying.update((prevAudio): AudioPlayingState => {
    prevAudio?.htmlEl.pause()

    const newAudio = new Audio(getStreamUri(trackId))
    newAudio.play().catch((e) => alert('Failed to play audio: ' + (e instanceof Error ? e.message : '<unknown error>')))

    audioProgress.set(0)

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
    console.warn('Tried to set audio progress while no audio is playing')
    return
  }

  playing.htmlEl.currentTime = seconds
}
