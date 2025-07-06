<script setup lang="ts">
import DistractionFreeTogglable from '@/components/atoms/DistractionFreeTogglable.vue'
import NavBar from '@/components/molecules/NavBar.vue'
import Notifications from '@/components/molecules/Notifications.vue'
import ContextMenu from '@/components/organisms/ContextMenu.vue'
import { setPlayingAudioProgressRelative, toggleAudioPlayback } from '@/global/stores/audio-player'
import { NotificationLevel, showNotification } from '@/global/stores/notifications'
import { playNextTrack, playPreviousTrackOrRewind, restorePlayQueue } from '@/global/stores/play-queue'
import { InputHandlingResult, LogLevel, handleInput, setupNavigable, watchLongPressForKeys } from '@/navigable'
import NavigableList from '@/navigable/vue/components/NavigableList.vue'
import router from '@/router'
import { onMounted } from 'vue'
import { log } from './global/stores/debugger'

onMounted(() => {
	setupNavigable({
		log(level, message, error) {
			log(level, message, error)

			if (level === LogLevel.Warn) {
				showNotification(NotificationLevel.Warn, message)
			}

			if (level === LogLevel.Error || level === LogLevel.Fatal) {
				showNotification(NotificationLevel.Error, message)
			}
		},
	})

	window.addEventListener('error', (err) => {
		showNotification(NotificationLevel.Error, `JavaScript runtime error:\n\n${err.message}`)
		console.error(err)
	})

	window.addEventListener('unhandledrejection', (e) => {
		showNotification(
			NotificationLevel.Error,
			`JavaScript unhandled Promise rejection:\n\n${typeof e === 'string' ? e : e instanceof Error ? e.message : '<no specified message>'}`,
		)
	})

	restorePlayQueue()
})

watchLongPressForKeys(['MediaPlayPause', 'MediaRewind', 'MediaFastForward'])

handleInput(({ key, longPress, ctrlKey, shiftKey }) => {
	if (key === 'MediaPlayPause') {
		if (!longPress) {
			toggleAudioPlayback()
		} else {
			router.push(router.currentRoute.value.name === 'now-playing' ? 'search' : 'now-playing')
		}
	} else if (key === 'MediaRewind' || (key === 'ArrowLeft' && shiftKey)) {
		if (!longPress) {
			setPlayingAudioProgressRelative(-10)
		} else {
			playPreviousTrackOrRewind()
		}
	} else if (key === 'MediaFastForward' || (key === 'ArrowRight' && shiftKey)) {
		if (!longPress) {
			setPlayingAudioProgressRelative(+10)
		} else {
			playNextTrack()
		}
	} else if (key === 'ArrowUp' && shiftKey) {
		playPreviousTrackOrRewind()
	} else if (key === 'ArrowDown' && shiftKey) {
		playNextTrack()
	} else if (key === 'F' && shiftKey && ctrlKey) {
		if (document.fullscreenElement) {
			document.exitFullscreen?.()
		} else {
			document.body.requestFullscreen()
		}
	} else {
		return InputHandlingResult.Propagate
	}

	return InputHandlingResult.Intercepted
}, 1)
</script>

<template>
	<div class="background fixed inset-0 -z-30" />

	<NavigableList @back-key="router.back()">
		<ContextMenu />
		<Notifications />

		<DistractionFreeTogglable>
			<NavBar :tabs="[
				{ label: 'Home', routeName: 'home' },
				{ label: 'History', routeName: 'history' },
				{ label: 'Now Playing', routeName: 'now-playing' },
				{
					label: 'Albums',
					routeName: 'albums',
					subMenu: [
						{ label: 'Album artists', routeName: 'album-artists' },
						{ label: 'Artists', routeName: 'artists' },
						{ label: 'Genres', routeName: 'genres' }
					],
				},
				{ label: 'Playlists', routeName: 'playlists' },
				{ label: 'Search', routeName: 'search' }]" />
		</DistractionFreeTogglable>

		<slot />
	</NavigableList>

</template>

<style scoped>
.background {
	background: linear-gradient(to bottom, #363636 0vh, #080808 33vh);
}
</style>
