import { LogListeningDocument, type AudioTrackFragment } from '@/graphql/generated/graphql'
import { log, logFatal, LogLevel } from '@/navigable'
import { readonly, ref, shallowRef } from 'vue'
import { getStreamUrl } from '../constants'
import { gqlClient } from '../urql-client'

const audioPlayer = shallowRef<HTMLAudioElement | null>(null)
const audioProgress = shallowRef<number | null>(null)
const audioPaused = shallowRef<boolean | null>(null)
const audioListeningDuration = ref<{ track: AudioTrackFragment; duration_s: number } | null>(null)

export const readableAudioProgress = readonly(audioProgress)
export const readableAudioPaused = readonly(audioPaused)

async function _changeCurrentPlayingTrack(resetAs: AudioTrackFragment | null): Promise<void> {
	if (audioListeningDuration.value === null) {
		if (!resetAs) {
			logFatal('Got no initial track')
		}

		audioListeningDuration.value = { track: resetAs, duration_s: 0 }

		return
	}

	const { track, duration_s: prevDurationS } = audioListeningDuration.value

	// Reset immediately to avoid skipping it accidentally if an error occurs below
	audioListeningDuration.value = { track: resetAs ?? track, duration_s: 0 }

	const duration_s = Math.floor(prevDurationS)

	log(
		LogLevel.Info,
		`Registering listening duration of ${duration_s} seconds for track with ID: ${track.id} | ${track.metadata.tags.title}`,
	)

	const { error } = await gqlClient.mutation(LogListeningDocument, { trackId: track.id, duration_s })

	if (error) {
		log(LogLevel.Error, 'Failed to register listening duration', error)
	}
}

export function startAudioPlayer(track: AudioTrackFragment, nextHandler: () => void, play = true) {
	const prev = audioPlayer.value

	if (prev && !prev.paused) {
		prev.pause()
	}

	audioPaused.value = false
	audioProgress.value = 0

	_changeCurrentPlayingTrack(track)

	log(LogLevel.Info, `Started playing track with ID: ${track.id} | ${track.metadata.tags.title}`)

	let lastTimeUpdate = 0

	const newAudio = new Audio(getStreamUrl(track.id))

	newAudio.addEventListener('error', (e) => log(LogLevel.Error, 'Failed to load audio track', e))
	newAudio.addEventListener('play', () => {
		audioPaused.value = false
	})
	newAudio.addEventListener('pause', () => {
		audioPaused.value = true
	})
	newAudio.addEventListener('ended', nextHandler)
	newAudio.addEventListener('timeupdate', () => {
		const currentTime = newAudio.currentTime
		audioProgress.value = Math.round(currentTime)

		if (
			// Don't increase listening duration in case of jump
			currentTime < lastTimeUpdate + 3 &&
			// Nor when going back
			currentTime > lastTimeUpdate
		) {
			const d = audioListeningDuration.value

			audioListeningDuration.value =
				d !== null
					? { track: d.track, duration_s: d.duration_s + (currentTime - lastTimeUpdate) }
					: logFatal('Tried to increment null audio listening duration!')
		}

		lastTimeUpdate = currentTime
	})

	if (play !== false) {
		newAudio.play().catch((e: unknown) => log(LogLevel.Error, 'Failed to play audio', e))
	}

	audioPlayer.value = newAudio
}

export function setPlayingAudioProgress(seconds: number) {
	const player = audioPlayer.value

	if (!player) {
		log(LogLevel.Warn, 'Tried to set audio progress while no audio is playing')
		return
	}

	player.currentTime = seconds

	log(LogLevel.Debug, `Set absolute audio progress: ${humanReadableDuration(seconds)}`)
}

export function setPlayingAudioProgressRelative(relativeSeconds: number) {
	const player = audioPlayer.value

	if (!player) {
		log(LogLevel.Warn, 'Tried to set audio progress while no audio is playing')
		return
	}

	const prevTime = player.currentTime

	player.currentTime = player.currentTime + relativeSeconds

	log(
		LogLevel.Debug,
		`Set relative audio progress: ${relativeSeconds >= 0 ? '+' : ''}${relativeSeconds}s (${prevTime}s => ${
			player.currentTime
		}s)`,
	)
}

export function toggleAudioPlayback() {
	const player = audioPlayer.value

	if (!player) {
		log(LogLevel.Warn, 'Tried to toggle audio playback while no audio is playing')
		return
	}

	log(LogLevel.Info, 'Toggled audio playback')

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
	const player = audioPlayer.value

	if (!player) {
		if (ignoreNoPlayer !== true) {
			log(LogLevel.Warn, 'Tried to stop audio playback while no audio is playing')
		}

		return
	}

	_changeCurrentPlayingTrack(null)

	if (!player.paused) {
		log(LogLevel.Info, 'Stopped audio playback')
		player.pause()
	} else if (ignoreAlreadyPaused !== true) {
		log(LogLevel.Info, 'Tried to stop audio playback, but it was already paused')
	}

	if (!justPause) {
		player.currentTime = 0
	}
}

export function humanReadableDuration(seconds: number): string {
	const hours = Math.floor(seconds / 3600)
	// biome-ignore lint/style/noParameterAssign: <explanation>
	seconds -= hours * 3600

	const minutes = Math.floor(seconds / 60)
	// biome-ignore lint/style/noParameterAssign: <explanation>
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
