import { derived, Readable, Writable } from 'svelte/store'
import { AudioTrackFragment } from '../graphql/generated'

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

export function hasMinimumRating(track: AudioTrackFragment, min: number): boolean {
	const rating = track.appOnlyRating ?? track.metadata.tags.rating
	return rating !== null && rating !== undefined && rating >= min
}

export function filterMap<T, U>(array: T[], track: (value: T) => U | null | undefined): U[] {
	const out: U[] = []

	for (const item of array) {
		const mapped = track(item)

		if (mapped !== null && mapped !== undefined) {
			out.push(mapped)
		}
	}

	return out
}

export function dedup<T>(array: T[]): T[] {
	return [...new Set(array)]
}

export function isDefined<T>(value: T | null | undefined): value is T {
	return value !== null && value !== undefined
}

export function shuffle<T>(array: T[]): T[] {
	return [...array].sort(() => (Math.random() > 0.5 ? 1 : -1))
}

export function readonly<T>(store: Writable<T>): Readable<T> {
	return derived(store, (_) => _)
}
