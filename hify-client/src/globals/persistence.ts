import { logDebug, logError, logInfo } from '@stores/debugger'
import type { PlayQueue } from '@stores/play-queue'
import { array, boolean, nullable, number, object, parse, string } from 'valibot'

const PLAY_QUEUE_LOCAL_STORAGE_KEY = 'hifyClient-playQueue'

export function persistPlayQueue(playQueue: PlayQueue) {
	localStorage.setItem(PLAY_QUEUE_LOCAL_STORAGE_KEY, JSON.stringify(playQueue))
	logDebug('Persisted play queue')
}

export function loadPlayQueue(): PlayQueue | null {
	logInfo('Loading play queue...')

	const saved = localStorage.getItem(PLAY_QUEUE_LOCAL_STORAGE_KEY)

	if (saved === null) {
		return null
	}

	let parsed: unknown

	try {
		parsed = JSON.parse(saved)
	} catch (e: unknown) {
		logError(`Failed to deserialize persisted play queue: ${e instanceof Error ? e.message : '<unknown error>'}`)
		localStorage.removeItem(PLAY_QUEUE_LOCAL_STORAGE_KEY)
		return null
	}

	let validated: PlayQueue

	try {
		validated = parse(PlayQueueSchema, parsed)
	} catch (e) {
		logError(`Failed to deserialize persisted play queue: ${e instanceof Error ? e.message : '<unknown error>'}`)
		localStorage.removeItem(PLAY_QUEUE_LOCAL_STORAGE_KEY)
		return null
	}

	logInfo('Successfully loaded persisted play queue')
	return validated
}

const PlayQueueSchema = object({
	tracks: array(
		object({
			idInQueue: string(),
			id: string(),
			metadata: object({
				duration: number(),
				tags: object({
					title: string(),
					album: object({
						id: string(),
						name: string(),
						albumArtists: array(
							object({
								id: string(),
								name: string(),
							}),
						),
					}),
					artists: array(
						object({
							id: string(),
							name: string(),
						}),
					),
					genres: array(
						object({
							id: string(),
							name: string(),
						}),
					),
				}),
			}),
		}),
	),
	position: nullable(number()),
	fromMixId: nullable(string()),
	isMixFinished: boolean(),
})
