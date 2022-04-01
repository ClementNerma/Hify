import { writable } from 'svelte/store'

export const playingAudio = writable<HTMLAudioElement | null>(null)

export function playAudio(url: string) {
  playingAudio.update((prevAudio) => {
    prevAudio?.pause()

    const newAudio = new Audio(url)
    newAudio.play().catch((e) => alert('Failed to play audio: ' + (e instanceof Error ? e.message : '<unknown error>')))

    return newAudio
  })
}
