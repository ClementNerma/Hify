import {
	type AudioTrackFragment,
	GenerateMixDocument,
	GetNextTracksOfMixDocument,
	type MixParams,
	SelectTracksDocument,
} from '@/graphql/generated/graphql'
import { LogLevel, log, logFatal } from '@/navigable'
import router from '@/router'
import { computed, ref, watch } from 'vue'
import { EXTEND_MIX_TRACKS_QTY, LARGE_MIX_TRACKS_QTY } from '../constants'
import { loadPlayQueue, persistPlayQueue } from '../persistence'
import { gqlClient } from '../urql-client'
import { swapInArray } from '../utils'
import { readableAudioProgress, replayTrack, startAudioPlayer, stopAudioPlayer } from './audio-player'
import { NotificationLevel, showNotification } from './notifications'

export type PlayQueue = {
	tracks: QueuedTrack[]
	position: number | null
	fromMixId: string | null
	isMixFinished: boolean
}

export type QueuedTrack = AudioTrackFragment & Readonly<{ idInQueue: string }>

const playQueue = ref<PlayQueue>({
	tracks: [],
	position: null,
	fromMixId: null,
	isMixFinished: false,
})

export const readablePlayQueue = computed(() => playQueue.value)

export const currentTrack = computed(() => {
	const { position, tracks } = readablePlayQueue.value
	return position !== null ? tracks[position] : null
})

export const PREVIOUS_TRACK_OR_REWIND_THRESOLD_SECONDS = 5

watch(
	playQueue,
	async ({ tracks, position, fromMixId, isMixFinished }) => {
		const currentTracks = tracks.map((track) => track.id)

		if (fromMixId === null || position === null || position <= tracks.length - 3 || isMixFinished) {
			return
		}

		// Generate a new mix from the previous settings, but exclude the tracks that are already in the current queue
		// to avoid getting duplicate tracks
		const nextTracks = await getNextTracksOfMix(fromMixId)

		// If playlist has changed, don't modify it
		if (tracks.length !== currentTracks.length || tracks.find((track, i) => currentTracks[i] !== track.id)) {
			return
		}

		// Otherwise, append the new tracks!
		playQueue.value = {
			tracks: tracks.concat(nextTracks.map(makeQueuedTrack)),
			position,
			fromMixId,
			isMixFinished: tracks.length === 0,
		}
	},
	{
		deep: true,
	},
)

function makeQueuedTrack(track: AudioTrackFragment): QueuedTrack {
	return { ...track, idInQueue: Math.random().toString() }
}

export function playNewQueueFromBeginning(tracks: AudioTrackFragment[], fromMixId: string | null): void {
	playTrackFromNewQueue(tracks, 0, fromMixId)
}

export function playTrackFromNewQueue(tracks: AudioTrackFragment[], position: number, fromMixId: string | null): void {
	if (tracks.length === 0) {
		showNotification(NotificationLevel.Error, 'Cannot play queue as it contains no track')
		return
	}

	playQueue.value = {
		tracks: tracks.map(makeQueuedTrack),
		position,
		fromMixId,
		isMixFinished: false,
	}

	startAudioPlayer(tracks[position], playNextTrack)
}

export function playTrackFromCurrentQueue(position: number): void {
	startAudioPlayer(playQueue.value.tracks[position], playNextTrack)
	playQueue.value.position = position
}

export function playPreviousTrackOrRewind(): void {
	log(LogLevel.Info, 'Going to play previous track or rewind...')

	const progress = readableAudioProgress.value

	if (progress !== null && progress > PREVIOUS_TRACK_OR_REWIND_THRESOLD_SECONDS) {
		replayTrack()
	} else {
		const { tracks, position, fromMixId, isMixFinished } = playQueue.value

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
			log(LogLevel.Info, `Playing previous track at position: ${newPosition.toString()}`)
		} else {
			log(LogLevel.Info, 'No previous track to play')
		}

		playQueue.value = {
			tracks,
			position: newPosition ?? position,
			fromMixId,
			isMixFinished,
		}
	}
}

export function playNextTrack(): void {
	log(LogLevel.Info, 'Going to play next track...')

	const { tracks, position, fromMixId, isMixFinished } = playQueue.value

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
		log(LogLevel.Info, `Playing next track at position: ${newPosition.toString()}`)
	} else {
		log(LogLevel.Info, 'No more track to play')
	}

	playQueue.value = { tracks, position: newPosition, fromMixId, isMixFinished }
}

export function enqueue(list: AudioTrackFragment[], where: 'next' | 'end'): void {
	log(LogLevel.Info, `Queuing ${list.length} tracks as ${where}`)

	const { position, tracks, fromMixId, isMixFinished } = playQueue.value

	const toQueue = list.map(makeQueuedTrack)

	playQueue.value =
		position === null
			? { position: null, tracks: toQueue, fromMixId, isMixFinished }
			: where === 'next'
				? {
						position,
						tracks: tracks
							.slice(0, position + 1)
							.concat(toQueue)
							.concat(tracks.slice(position + 1)),
						fromMixId,
						isMixFinished,
					}
				: where === 'end'
					? {
							position,
							tracks: tracks.concat(toQueue),
							fromMixId,
							isMixFinished,
						}
					: logFatal('Unreachable')
}

export function removeFromQueue(index: number): void {
	log(LogLevel.Info, `Removing track n°${index + 1} from queue`)

	const { position, tracks, fromMixId, isMixFinished } = playQueue.value

	if (!Object.prototype.hasOwnProperty.call(tracks, index)) {
		log(LogLevel.Warn, 'Cannot remove track from queue as the provided position does not exist')
		return
	}

	if (index === position) {
		log(LogLevel.Warn, 'Cannot remove track from queue as it is the currently-playing track')
		return
	}

	playQueue.value = {
		position: position === null ? null : index < position ? position - 1 : position,
		tracks: tracks.slice(0, index).concat(tracks.slice(index + 1)),
		fromMixId,
		isMixFinished,
	}
}

export function moveTrackPositionInQueue(index: number, newIndex: number): void {
	log(LogLevel.Info, `Moving track n°${index + 1} to position ${newIndex + 1} in queue`)

	const { position, tracks, fromMixId, isMixFinished } = playQueue.value

	if (!Object.prototype.hasOwnProperty.call(tracks, index)) {
		log(LogLevel.Warn, 'Cannot move track in  queue as the provided position does not exist')
		return
	}

	if (!Object.prototype.hasOwnProperty.call(tracks, newIndex)) {
		log(LogLevel.Warn, 'Cannot move track in queue as the provided target position does not exist')
		return
	}

	playQueue.value = {
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
}

export async function generateAndPlayMix(params: MixParams): Promise<void> {
	const { data, error } = await gqlClient.mutation(GenerateMixDocument, {
		params,
		maxTracks: LARGE_MIX_TRACKS_QTY,
	})

	if (!data) {
		logFatal('Failed to generate mix', error)
	}

	const { mixId, firstTracks } = data.generateMix

	playNewQueueFromBeginning(firstTracks, mixId)

	router.push({ name: 'now-playing' })
}

export async function getNextTracksOfMix(mixId: string): Promise<AudioTrackFragment[]> {
	const mix = await gqlClient.query(GetNextTracksOfMixDocument, {
		mixId,
		maxTracks: EXTEND_MIX_TRACKS_QTY,
	})

	if (!mix.data) {
		logFatal('Failed to resume mix')
	}

	return mix.data.getNextTracksOfMix
}

export function restorePlayQueue() {
	const persistedPlayQueue = loadPlayQueue()

	if (persistedPlayQueue === null) {
		return
	}

	const { tracksId, position, fromMixId, isMixFinished } = persistedPlayQueue

	gqlClient
		.query(SelectTracksDocument, {
			inIds: tracksId,
		})
		.then(({ data, error }) => {
			if (!data) {
				log(LogLevel.Error, 'Failed to get tracks from persisted play queue', error)
				return
			}

			playQueue.value = {
				tracks: data.selectTracks.map(makeQueuedTrack),
				position,
				fromMixId,
				isMixFinished,
			}
		})
}

watch(playQueue, persistPlayQueue, { deep: true })
