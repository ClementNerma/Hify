<script setup lang="ts">
import { setPlayingAudioProgressRelative, toggleAudioPlayback } from '@/global/stores/audio-player'
import { playNextTrack, playPreviousTrackOrRewind } from '@/global/stores/play-queue'

import NavigableList from '@/navigable/headless/NavigableList/NavigableList.vue'
import NavigablePage from '@/navigable/headless/NavigablePage/NavigablePage.vue'

import DistractionFreeTogglable from '@/components/atoms/DistractionFreeTogglable.vue'
import { showErrorDialog } from '@/components/molecules/ErrorDialog'
import NavBar from '@/components/molecules/NavBar.vue'
import NavigableWithHandlers from '@/navigable/headless/NavigableWithHandlers/NavigableWithHandlers.vue'
import { KeyPressHandling, handleInput, registerLongPressableKeys } from '@/navigable/input-manager'
import ContextMenu from '@/navigable/ui/molecules/ContextMenu/ContextMenu.vue'
import ErrorDialog from '@/components/molecules/ErrorDialog.vue'
import router from '@/router'
import { onMounted } from 'vue'

registerLongPressableKeys('MediaPlayPause', 'MediaRewind', 'MediaFastForward', 'Escape')

handleInput((key, long) => {
    switch (key) {
        case 'MediaPlayPause':
        case 'p':
            if (!long) {
                toggleAudioPlayback()
            } else {
                router.push(router.currentRoute.value.name === 'now-playing' ? 'search' : 'now-playing')
            }

            break

        case 'MediaRewind':
        case 'r':
            if (!long) {
                setPlayingAudioProgressRelative(-10)
            } else {
                playPreviousTrackOrRewind()
            }

            break

        case 'MediaFastForward':
        case 'f':
            if (!long) {
                setPlayingAudioProgressRelative(+10)
            } else {
                playNextTrack()
            }

            break

        default:
            return
    }

    return KeyPressHandling.Propagate
})

onMounted(() => {
    window.addEventListener('error', (err) => {
        console.error(err)
        showErrorDialog('JavaScript runtime error', err.message)
    })

    window.addEventListener('unhandledrejection', () => {
        showErrorDialog('JavaScript unhandled Promise rejection', '<unknown message>')
    })
})

const win = window
</script>

<template>
    <div class="background fixed inset-0 -z-30" />

    <NavigablePage>
        <NavigableWithHandlers @back="router.back()" @long-back="win.location.reload()">
            <NavigableList>
                <ContextMenu />
                <ErrorDialog />

                <DistractionFreeTogglable>
                    <NavigableWithHandlers @long-press="router.push({ name: 'devtools' })">
                        <NavBar :tabs="[
                            { label: 'Home', routeName: 'home' },
                            { label: 'History', routeName: 'history' },
                            { label: 'Now Playing', routeName: 'now-playing' },
                            {
                                label: 'Albums',
                                routeName: 'albums',
                                subMenu: [
                                    { label: 'Artists', routeName: 'artists' },
                                    { label: 'Genres', routeName: 'genres' }
                                ],
                            },
                            { label: 'Playlists', routeName: 'playlists' },
                            { label: 'Search', routeName: 'search' }]" />
                    </NavigableWithHandlers>
                </DistractionFreeTogglable>

                <slot />
            </NavigableList>
        </NavigableWithHandlers>
    </NavigablePage>
</template>

<style scoped>
.background {
    background: linear-gradient(to bottom, #363636 0%, #080808 33%);
}
</style>
