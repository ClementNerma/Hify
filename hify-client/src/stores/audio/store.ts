import { derived, writable } from 'svelte/store'
import { getStreamUri } from '../../rest-api'
import { AsyncAudioTrack, AudioTrackQuery } from './AudioTrack.generated'

type AudioPlayingState = {
  htmlEl: HTMLAudioElement
  trackId: string
  trackInfos: Promise<AudioTrackQuery['track']>
}

const audioPlaying = writable<AudioPlayingState | null>(null)

export const readableAudioPlaying = derived(audioPlaying, (_) => _)

export function playTrack(trackId: string) {
  audioPlaying.update((prevAudio): AudioPlayingState => {
    prevAudio?.htmlEl.pause()

    const newAudio = new Audio(getStreamUri(trackId))
    newAudio.play().catch((e) => alert('Failed to play audio: ' + (e instanceof Error ? e.message : '<unknown error>')))

    return {
      htmlEl: newAudio,
      trackId,
      trackInfos: AsyncAudioTrack({
        variables: {
          trackId,
        },
      }).then((res) => res.data.track),
    }
  })
}
