import { derived, get, writable } from 'svelte/store'
import { generateMix } from '../atoms/MixButton/MixGenerator'
import { EXTEND_MIX_TRACKS_QTY } from '../constants'
import { readonly, swapInArray } from '../globals/utils'
import { AudioTrackFragment, MixParams } from '../graphql/generated'
import { readableAudioProgress, replayTrack, startAudioPlayer, stopAudioPlayer } from './audio-player'
import { logInfo, logWarn } from './debugger'

type PlayQueue = {
	tracks: QueuedTrack[]
	position: number | null
	fromMixParams: MixParams | null
}

type QueuedTrack = AudioTrackFragment & Readonly<{ idInQueue: string }>

const playQueue = writable<PlayQueue>({
	tracks: [],
	position: null,
	fromMixParams: null,
})

export const readablePlayQueue = readonly(playQueue)
export const currentTrack = derived(playQueue, ({ tracks, position }) => position !== null && tracks[position])
export const queuePosition = derived(playQueue, ({ position }) => position)

export const PREVIOUS_TRACK_OR_REWIND_THRESOLD_SECONDS = 5

playQueue.subscribe(async ({ tracks, position, fromMixParams }) => {
	const currentTracks: string[] = tracks.map((track) => track.id)

	if (!fromMixParams || position === null || position <= tracks.length - 3) {
		return
	}

	// Generate a new mix from the previous settings, but exclude the tracks that are already in the current queue
	// to avoid getting duplicate tracks
	const res = await generateMix(
		{ ...fromMixParams, excludeTracks: (fromMixParams.excludeTracks ?? []).concat(currentTracks) },
		EXTEND_MIX_TRACKS_QTY,
	)

	// Prepare new tracks for queuing
	const nextTracks = res.data.generateMix.map(makeQueuedTrack)

	// Update the queue
	playQueue.update(({ tracks, position, fromMixParams }) => {
		// If playlist has changed, don't modify it
		if (tracks.length !== currentTracks.length || tracks.find((track, i) => currentTracks[i] !== track.id)) {
			return { tracks, position, fromMixParams }
		}

		// Otherwise, append the new tracks!
		return { tracks: tracks.concat(nextTracks), position, fromMixParams }
	})
})

function makeQueuedTrack(track: AudioTrackFragment): QueuedTrack {
	return { ...track, idInQueue: Math.random().toString() }
}

export function playNewQueueFromBeginning(tracks: AudioTrackFragment[], fromMixParams: MixParams | null): void {
	playQueue.set({ tracks: tracks.map(makeQueuedTrack), position: 0, fromMixParams })
	startAudioPlayer(tracks[0], playNextTrack)
}

export function playTrackFromNewQueue(
	tracks: AudioTrackFragment[],
	position: number,
	fromMixParams: MixParams | null,
): void {
	playQueue.set({ tracks: tracks.map(makeQueuedTrack), position, fromMixParams })
	startAudioPlayer(tracks[position], playNextTrack)
}

export function playTrackFromCurrentQueue(position: number): void {
	playQueue.update(({ tracks, fromMixParams }) => {
		startAudioPlayer(tracks[position], playNextTrack)
		return { tracks, position, fromMixParams }
	})
}

export function playPreviousTrackOrRewind(): void {
	logInfo('Going to play previous track or rewind...')

	const progress = get(readableAudioProgress)

	if (progress !== null && progress > PREVIOUS_TRACK_OR_REWIND_THRESOLD_SECONDS) {
		replayTrack()
	} else {
		playQueue.update(({ tracks, position, fromMixParams }) => {
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

			return { tracks, position: newPosition ?? position, fromMixParams }
		})
	}
}

export function playNextTrack(): void {
	logInfo('Going to play next track...')

	playQueue.update(({ tracks, position, fromMixParams }) => {
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

		return { tracks, position: newPosition, fromMixParams }
	})
}

export function enqueue(list: AudioTrackFragment[], where: 'next' | 'end'): void {
	logInfo(`Queuing ${list.length} tracks as ${where}`)

	playQueue.update(({ position, tracks, fromMixParams }) => {
		const toQueue = list.map(makeQueuedTrack)

		if (position === null) {
			return { position: null, tracks: toQueue, fromMixParams }
		}

		switch (where) {
			case 'next':
				return {
					position,
					tracks: tracks
						.slice(0, position + 1)
						.concat(toQueue)
						.concat(tracks.slice(position + 1)),
					fromMixParams,
				}

			case 'end':
				return {
					position,
					tracks: tracks.concat(toQueue),
					fromMixParams,
				}
		}
	})
}

export function removeFromQueue(index: number): void {
	logInfo(`Removing track n°${index + 1} from queue`)

	playQueue.update(({ position, tracks, fromMixParams }) => {
		if (!Object.prototype.hasOwnProperty.call(tracks, index)) {
			logWarn('Cannot remove track from queue as the provided position does not exist')
			return { position, tracks, fromMixParams }
		}

		if (index === position) {
			logWarn('Cannot remove track from queue as it is the currently-playing track')
			return { position, tracks, fromMixParams }
		}

		return {
			position: position === null ? null : index < position ? position - 1 : position,
			tracks: tracks.slice(0, index).concat(tracks.slice(index + 1)),
			fromMixParams,
		}
	})
}

export function moveTrackPositionInQueue(index: number, newIndex: number): void {
	logInfo(`Moving track n°${index + 1} to position ${newIndex + 1} in queue`)

	playQueue.update(({ position, tracks, fromMixParams }) => {
		if (!Object.prototype.hasOwnProperty.call(tracks, index)) {
			logWarn('Cannot move track in  queue as the provided position does not exist')
			return { position, tracks, fromMixParams }
		}

		if (!Object.prototype.hasOwnProperty.call(tracks, newIndex)) {
			logWarn('Cannot move track in queue as the provided target position does not exist')
			return { position, tracks, fromMixParams }
		}

		return {
			tracks: swapInArray(tracks, index, newIndex),
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
			fromMixParams,
		}
	})
}
