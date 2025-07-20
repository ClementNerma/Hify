import * as v from 'valibot'
import { LogLevel, log } from '@/navigable'
import type { PlayQueue } from './stores/play-queue'

const PLAY_QUEUE_LOCAL_STORAGE_KEY = 'hifyClient-playQueue'

type PersistedPlayQueue = {
	tracksId: string[]
	position: number | null
	fromMixId: string | null
	isMixFinished: boolean
}

export function persistPlayQueue(playQueue: PlayQueue) {
	const { tracks, position, fromMixId, isMixFinished } = playQueue

	const state: PersistedPlayQueue = {
		tracksId: tracks.map((track) => track.id),
		position,
		fromMixId,
		isMixFinished,
	}

	localStorage.setItem(PLAY_QUEUE_LOCAL_STORAGE_KEY, JSON.stringify(state))

	log(LogLevel.Debug, 'Persisted play queue')
}

export function loadPlayQueue(): PersistedPlayQueue | null {
	log(LogLevel.Debug, 'Loading play queue...')

	const saved = localStorage.getItem(PLAY_QUEUE_LOCAL_STORAGE_KEY)

	if (saved === null) {
		return null
	}

	let parsed: unknown

	try {
		parsed = JSON.parse(saved)
	} catch (e: unknown) {
		log(LogLevel.Error, `Failed to parse persisted play queue: ${e instanceof Error ? e.message : '<unknown error>'}`)
		console.error(saved)
		localStorage.removeItem(PLAY_QUEUE_LOCAL_STORAGE_KEY)
		return null
	}

	let validated: PersistedPlayQueue

	try {
		validated = v.parse(PersistedPlayQueueSchema, parsed)
	} catch (e) {
		log(
			LogLevel.Error,
			`Failed to destructurize persisted play queue: ${e instanceof Error ? e.message : '<unknown error>'}`,
		)
		console.error(parsed)
		localStorage.removeItem(PLAY_QUEUE_LOCAL_STORAGE_KEY)
		return null
	}

	log(LogLevel.Debug, 'Successfully loaded persisted play queue')

	return validated
}

const PersistedPlayQueueSchema = v.object({
	tracksId: v.array(v.string()),
	position: v.nullable(v.number()),
	fromMixId: v.nullable(v.string()),
	isMixFinished: v.boolean(),
})
