import { AudioTrackFragment } from './graphql/generated'

export function bind<T, U>(data: T, callback: (value: T) => U): () => U {
  return () => callback(data)
}

export function twoDigits(num: number): string {
  return num.toString().padStart(2, '0')
}

// TODO: International formatting
export function formatDate({ day, month, year }: NonNullable<AudioTrackFragment['metadata']['tags']['date']>): string {
  return day !== null && day !== undefined && month !== null && month !== undefined
    ? `${twoDigits(day)}/${twoDigits(month)}/${year}`
    : month !== null && month !== undefined
    ? `${twoDigits(month)}/${year}`
    : year.toString()
}

export function hasMinimumNote(track: AudioTrackFragment, min: number): boolean {
  const note = track.metadata.tags.note
  return note !== null && note !== undefined && note >= min
}
