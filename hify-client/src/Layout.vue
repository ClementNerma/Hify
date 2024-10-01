<script setup lang="ts">
import DistractionFreeTogglable from '@/components/atoms/DistractionFreeTogglable.vue'
import NavBar from '@/components/molecules/NavBar.vue'
import router from '@/router'
import { onMounted } from 'vue'
import NavigableList from '@/navigable/vue/components/NavigableList.vue'
import ContextMenu from '@/components/organisms/ContextMenu.vue'
import { InputHandlingResult, handleInput, setupNavigable, watchLongPressForKeys } from '@/navigable'
import Notifications from '@/components/molecules/Notifications.vue'
import { setPlayingAudioProgressRelative, toggleAudioPlayback } from '@/global/stores/audio-player'
import { playNextTrack, playPreviousTrackOrRewind } from '@/global/stores/play-queue'
import { NotificationLevel, showNotification } from '@/global/stores/notifications'

onMounted(() => {
    setupNavigable({
        logFatal(message) {
            showNotification(NotificationLevel.Error, message)
            throw new Error(message)
        },

        logWarn(message) {
            showNotification(NotificationLevel.Warn, message)
            console.warn(message)
        }
    })

    window.addEventListener('error', (err) => {
        showNotification(NotificationLevel.Error, `JavaScript runtime error:\n\n${err.message}`)
        console.error(err)
    })

    window.addEventListener('unhandledrejection', (e) => {
        showNotification(NotificationLevel.Error, `JavaScript unhandled Promise rejection:\n\n${typeof e === 'string' ? e : e instanceof Error ? e.message : '<no specified message>'}`)
    })
})

watchLongPressForKeys(['MediaPlayPause', 'MediaRewind', 'MediaFastForward'])

handleInput((key, long) => {
    switch (key) {
        case 'MediaPlayPause':
            if (!long) {
                toggleAudioPlayback()
            } else {
                router.push(router.currentRoute.value.name === 'now-playing' ? 'search' : 'now-playing')
            }

            break

        case 'MediaRewind':
            if (!long) {
                setPlayingAudioProgressRelative(-10)
            } else {
                playPreviousTrackOrRewind()
            }

            break

        case 'MediaFastForward':
            if (!long) {
                setPlayingAudioProgressRelative(+10)
            } else {
                playNextTrack()
            }

            break

        default:
            return InputHandlingResult.Propagate
    }

    return InputHandlingResult.Intercepted
})

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
