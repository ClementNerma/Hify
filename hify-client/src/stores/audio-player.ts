import { get, writable } from 'svelte/store'
import { getStreamUri } from '../globals/rest-api'
import { readonly } from '../globals/utils'
import { AudioTrackFragment, LogListening } from '../graphql/generated'
import { logDebug, logError, logFatal, logInfo, logWarn } from './debugger'

const audioPlayer = writable<HTMLAudioElement | null>(null)
const audioProgress = writable<number | null>(null)
const audioPaused = writable<boolean | null>(null)
const audioListeningDuration = writable<{ track: AudioTrackFragment; duration_s: number } | null>(null)

export const readableAudioProgress = readonly(audioProgress)
export const readableAudioPaused = readonly(audioPaused)

function _newListeningSession(resetAs: AudioTrackFragment | null): void {
	audioListeningDuration.update((prevDuration) => {
		if (prevDuration !== null) {
			const { track } = prevDuration
			const duration_s = Math.floor(prevDuration.duration_s)

			logInfo(
				`Registering listening duration of ${duration_s} seconds for track ${track.id} ('${track.metadata.tags.title}')`,
			)

			LogListening({ variables: { trackId: track.id, duration_s } }).catch((e: unknown) =>
				logError('Failed to register listening duration', e),
			)

			return { track: resetAs ?? track, duration_s: 0 }
		}

		if (!resetAs) {
			throw new Error('Got no track to reset as in duration watcher')
		}

		return { track: resetAs, duration_s: 0 }
	})
}

export function startAudioPlayer(track: AudioTrackFragment, nextHandler: () => void, play = true) {
	audioPlayer.update((prev): HTMLAudioElement => {
		if (prev && !prev.paused) {
			prev.pause()
		}

		audioPaused.set(false)
		audioProgress.set(0)

		_newListeningSession(track)

		logInfo(`Started playing track with ID: ${track.id} | ${track.metadata.tags.title}`)

		let lastTimeUpdate = 0

		const newAudio = new Audio(getStreamUri(track.id))

		newAudio.addEventListener('error', (e) => logError('Failed to load audio track', e))
		newAudio.addEventListener('play', () => audioPaused.set(false))
		newAudio.addEventListener('pause', () => audioPaused.set(true))
		newAudio.addEventListener('ended', nextHandler)
		newAudio.addEventListener('timeupdate', () => {
			const currentTime = newAudio.currentTime
			audioProgress.set(Math.round(currentTime))

			if (
				// Don't increase listening duration in case of jump
				currentTime < lastTimeUpdate + 3 &&
				// Nor when going back
				currentTime > lastTimeUpdate
			) {
				audioListeningDuration.update((d) =>
					d !== null
						? { track: d.track, duration_s: d.duration_s + (lastTimeUpdate - currentTime) }
						: logFatal('Tried to increment null audio listening duration!'),
				)
			}

			lastTimeUpdate = currentTime
		})

		if (play !== false) {
			newAudio.play().catch((e: unknown) => logError('Failed to play audio', e))
		}

		return newAudio
	})
}

export function setPlayingAudioProgress(seconds: number) {
	const player = get(audioPlayer)

	if (!player) {
		logWarn('Tried to set audio progress while no audio is playing')
		return
	}

	player.currentTime = seconds

	logDebug(`Set relative audio progress: ${humanReadableDuration(seconds)}`)
}

export function setPlayingAudioProgressRelative(relativeSeconds: number) {
	const player = get(audioPlayer)

	if (!player) {
		logWarn('Tried to set audio progress while no audio is playing')
		return
	}

	const prevTime = player.currentTime

	player.currentTime = player.currentTime + relativeSeconds

	logDebug(
		`Set relative audio progress: ${relativeSeconds >= 0 ? '+' : ''}${relativeSeconds}s (${prevTime}s => ${
			player.currentTime
		}s)`,
	)
}

export function toggleAudioPlayback() {
	const player = get(audioPlayer)

	if (!player) {
		logWarn('Tried to toggle audio playback while no audio is playing')
		return
	}

	logInfo('Toggled audio playback')

	if (player.paused) {
		player.play().catch((e) => alert(`Failed to resume audio: ${e instanceof Error ? e.message : '<unknown error>'}`))
	} else {
		player.pause()
	}
}

export function replayTrack() {
	setPlayingAudioProgress(0)
}

type StopAudioPlayerOptions = {
	justPause?: boolean
	ignoreAlreadyPaused?: boolean
	ignoreNoPlayer?: boolean
}

export function stopAudioPlayer({ justPause, ignoreAlreadyPaused, ignoreNoPlayer }: StopAudioPlayerOptions = {}) {
	const player = get(audioPlayer)

	if (!player) {
		if (ignoreNoPlayer !== true) {
			logWarn('Tried to stop audio playback while no audio is playing')
		}

		return
	}

	_newListeningSession(null)

	if (!player.paused) {
		logInfo('Stopped audio playback')
		player.pause()
	} else if (ignoreAlreadyPaused !== true) {
		logInfo('Tried to stop audio playback, but it was already paused')
	}

	if (!justPause) {
		player.currentTime = 0
	}
}

export function humanReadableDuration(seconds: number): string {
	const hours = Math.floor(seconds / 3600)
	seconds -= hours * 3600

	const minutes = Math.floor(seconds / 60)
	seconds -= minutes * 60

	const hoursPrefix = hours > 0 ? `${hours < 10 ? '0' : ''}${hours}:` : ''

	return `${hoursPrefix}${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`
}

document.addEventListener('visibilitychange', () => {
	if (document.visibilityState === 'hidden') {
		stopAudioPlayer({
			justPause: true,
			ignoreAlreadyPaused: true,
			ignoreNoPlayer: true,
		})
	}
})
