import { loadPlayQueue, persistPlayQueue } from '@globals/persistence'
import { readonly, swapInArray } from '@globals/utils'
import {
	AsyncSelectTracks,
	GenerateMix,
	GetNextTracksOfMix,
	type AudioTrackFragment,
	type MixParams,
} from '@graphql/generated'
import { showErrorDialog } from '@molecules/ErrorDialog/ErrorDialog'
import { navigate } from 'svelte-navigator'
import { derived, get, writable } from 'svelte/store'
import { EXTEND_MIX_TRACKS_QTY, LARGE_MIX_TRACKS_QTY } from '../constants'
import { ROUTES } from '../routes'
import { readableAudioProgress, replayTrack, startAudioPlayer, stopAudioPlayer } from './audio-player'
import { logFatal, logInfo, logWarn } from './debugger'

export type PlayQueue = {
	tracks: QueuedTrack[]
	position: number | null
	fromMixId: string | null
	isMixFinished: boolean
}

export type QueuedTrack = AudioTrackFragment & Readonly<{ idInQueue: string }>

const playQueue = writable<PlayQueue>({
	tracks: [],
	position: null,
	fromMixId: null,
	isMixFinished: false,
})

export const readablePlayQueue = readonly(playQueue)
export const currentTrack = derived(playQueue, ({ tracks, position }) => position !== null && tracks[position])
export const queuePosition = derived(playQueue, ({ position }) => position)

export const PREVIOUS_TRACK_OR_REWIND_THRESOLD_SECONDS = 5

playQueue.subscribe(async ({ tracks, position, fromMixId, isMixFinished }) => {
	const currentTracks: string[] = tracks.map((track) => track.id)

	if (fromMixId === null || position === null || position <= tracks.length - 3 || isMixFinished) {
		return
	}

	// Generate a new mix from the previous settings, but exclude the tracks that are already in the current queue
	// to avoid getting duplicate tracks
	const nextTracks = await getNextTracksOfMix(fromMixId)

	// Update the queue
	playQueue.update(({ tracks, position, fromMixId, isMixFinished }) => {
		// If playlist has changed, don't modify it
		if (tracks.length !== currentTracks.length || tracks.find((track, i) => currentTracks[i] !== track.id)) {
			return { tracks, position, fromMixId, isMixFinished }
		}

		// Otherwise, append the new tracks!
		return {
			tracks: tracks.concat(nextTracks.map(makeQueuedTrack)),
			position,
			fromMixId,
			isMixFinished: tracks.length === 0,
		}
	})
})

const persistedPlayQueue = loadPlayQueue()

playQueue.subscribe(persistPlayQueue)

if (persistedPlayQueue) {
	const { tracksId, position, fromMixId, isMixFinished } = persistedPlayQueue

	AsyncSelectTracks({
		variables: {
			inIds: tracksId,
		},
	}).then((res) => {
		playQueue.set({
			tracks: res.data.selectTracks.map(makeQueuedTrack),
			position,
			fromMixId,
			isMixFinished,
		})
	})
}

function makeQueuedTrack(track: AudioTrackFragment): QueuedTrack {
	return { ...track, idInQueue: Math.random().toString() }
}

export function playNewQueueFromBeginning(tracks: AudioTrackFragment[], fromMixId: string | null): void {
	playTrackFromNewQueue(tracks, 0, fromMixId)
}

export function playTrackFromNewQueue(tracks: AudioTrackFragment[], position: number, fromMixId: string | null): void {
	if (tracks.length === 0) {
		return showErrorDialog('Empty queue', 'Cannot play queue as it contains no track')
	}

	playQueue.set({ tracks: tracks.map(makeQueuedTrack), position, fromMixId, isMixFinished: false })
	startAudioPlayer(tracks[position], playNextTrack)
}

export function playTrackFromCurrentQueue(position: number): void {
	playQueue.update(({ tracks, fromMixId, isMixFinished }) => {
		startAudioPlayer(tracks[position], playNextTrack)
		return { tracks, position, fromMixId, isMixFinished }
	})
}

export function playPreviousTrackOrRewind(): void {
	logInfo('Going to play previous track or rewind...')

	const progress = get(readableAudioProgress)

	if (progress !== null && progress > PREVIOUS_TRACK_OR_REWIND_THRESOLD_SECONDS) {
		replayTrack()
	} else {
		playQueue.update(({ tracks, position, fromMixId, isMixFinished }) => {
			let newPosition: number | null

			if (position === null) {
				newPosition = null
			} else if (position === 0) {
				replayTrack()
				newPosition = null
			} else {
				newPosition = position - 1
			}

			if (newPosition !== null) {
				startAudioPlayer(tracks[newPosition], playNextTrack)
				logInfo(`Playing previous track at position: ${newPosition.toString()}`)
			} else {
				logInfo('No previous track to play')
			}

			return { tracks, position: newPosition ?? position, fromMixId, isMixFinished }
		})
	}
}

export function playNextTrack(): void {
	logInfo('Going to play next track...')

	playQueue.update(({ tracks, position, fromMixId, isMixFinished }) => {
		let newPosition: number | null

		if (position === null) {
			newPosition = tracks.length > 0 ? 0 : null
		} else if (position === tracks.length - 1) {
			stopAudioPlayer()
			newPosition = null
		} else {
			newPosition = position + 1
		}

		if (newPosition !== null) {
			startAudioPlayer(tracks[newPosition], playNextTrack)
			logInfo(`Playing next track at position: ${newPosition.toString()}`)
		} else {
			logInfo('No more track to play')
		}

		return { tracks, position: newPosition, fromMixId, isMixFinished }
	})
}

export function enqueue(list: AudioTrackFragment[], where: 'next' | 'end'): void {
	logInfo(`Queuing ${list.length} tracks as ${where}`)

	playQueue.update(({ position, tracks, fromMixId, isMixFinished }) => {
		const toQueue = list.map(makeQueuedTrack)

		if (position === null) {
			return { position: null, tracks: toQueue, fromMixId, isMixFinished }
		}

		switch (where) {
			case 'next':
				return {
					position,
					tracks: tracks
						.slice(0, position + 1)
						.concat(toQueue)
						.concat(tracks.slice(position + 1)),
					fromMixId,
					isMixFinished,
				}

			case 'end':
				return {
					position,
					tracks: tracks.concat(toQueue),
					fromMixId,
					isMixFinished,
				}
		}
	})
}

export function removeFromQueue(index: number): void {
	logInfo(`Removing track n°${index + 1} from queue`)

	playQueue.update(({ position, tracks, fromMixId, isMixFinished }) => {
		if (!Object.prototype.hasOwnProperty.call(tracks, index)) {
			logWarn('Cannot remove track from queue as the provided position does not exist')
			return { position, tracks, fromMixId, isMixFinished }
		}

		if (index === position) {
			logWarn('Cannot remove track from queue as it is the currently-playing track')
			return { position, tracks, fromMixId, isMixFinished }
		}

		return {
			position: position === null ? null : index < position ? position - 1 : position,
			tracks: tracks.slice(0, index).concat(tracks.slice(index + 1)),
			fromMixId,
			isMixFinished,
		}
	})
}

export function moveTrackPositionInQueue(index: number, newIndex: number): void {
	logInfo(`Moving track n°${index + 1} to position ${newIndex + 1} in queue`)

	playQueue.update(({ position, tracks, fromMixId, isMixFinished }) => {
		if (!Object.prototype.hasOwnProperty.call(tracks, index)) {
			logWarn('Cannot move track in  queue as the provided position does not exist')
			return { position, tracks, fromMixId, isMixFinished }
		}

		if (!Object.prototype.hasOwnProperty.call(tracks, newIndex)) {
			logWarn('Cannot move track in queue as the provided target position does not exist')
			return { position, tracks, fromMixId, isMixFinished }
		}

		return {
			tracks: swapInArray(tracks, index, newIndex),
			fromMixId,
			position:
				position === null
					? null
					: (index === position && newIndex < position) || // current track moved left, so move left
						  (index === position - 1 && newIndex === position) // left track moved right, so move left
					  ? position - 1
					  : (index === position && newIndex > position) || // current track moved right, so move right
							  (index === position + 1 && newIndex === position) // right track moved left, so move right
						  ? position + 1
						  : position,
			isMixFinished,
		}
	})
}

export async function generateAndPlayMix(params: MixParams): Promise<void> {
	const mix = await GenerateMix({
		variables: {
			params,
			maxTracks: LARGE_MIX_TRACKS_QTY,
		},
	})

	if (!mix.data) {
		return logFatal('Failed to generate mix')
	}

	const { mixId, firstTracks } = mix.data.generateMix

	playNewQueueFromBeginning(firstTracks, mixId)
	navigate(ROUTES.nowPlaying)
}

export async function getNextTracksOfMix(mixId: string): Promise<AudioTrackFragment[]> {
	const mix = await GetNextTracksOfMix({
		variables: {
			mixId,
			maxTracks: EXTEND_MIX_TRACKS_QTY,
		},
	})

	if (!mix.data) {
		return logFatal('Failed to resume mix')
	}

	return mix.data.getNextTracksOfMix
}
