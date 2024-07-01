import type { AudioTrackFragment } from '@/graphql/generated/graphql'
import type { VNodeRef } from 'vue'
import { useRoute } from 'vue-router'
import { logFatal } from './stores/debugger'

export function getRouteParam(name: string, fallback?: string): string {
	const { params } = useRoute()

	if (!Object.prototype.hasOwnProperty.call(params, name)) {
		if (fallback !== undefined) {
			return fallback
		}

		logFatal(`Parameter "${name}" was not found in current route`)
	}

	const value = params[name]

	if (typeof value !== 'string') {
		logFatal(`Please provide a single valid string value for route parameter "${name}"`)
	}

	return value
}

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
	const rating = track.computedRating
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

export function swapInArray<T>(array: T[], index: number, newIndex: number): T[] {
	if (!Object.prototype.hasOwnProperty.call(array, index) || !Object.prototype.hasOwnProperty.call(array, newIndex)) {
		throw new Error('Invalid indexes provided')
	}

	const newArray = [...array]
	const toMove = newArray[index]
	newArray[index] = newArray[newIndex]
	newArray[newIndex] = toMove

	return newArray
}

export function noParallel<F extends (...args: Args) => Promise<unknown>, Args extends unknown[]>(
	value: F,
): (...args: Args) => Promise<void> {
	let isRunning = false

	return async (...args) => {
		if (!isRunning) {
			isRunning = true

			try {
				value(...args)
			} finally {
				isRunning = false
			}
		}
	}
}

export function isApproachingGridEnd(itemIndex: number, columnsPerRow: number, totalItems: number): boolean {
	return itemIndex + columnsPerRow * 3 >= totalItems
}

export function bindRef<T extends object>(container: T, key: keyof T): VNodeRef {
	return (value) => {
		// biome-ignore lint/suspicious/noExplicitAny: <explanation>
		container[key] = value as any
	}
}
