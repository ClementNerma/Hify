import { toggleBlackOutModeFeature } from './global/bo-mode'
import { navigationManager } from './global/nav'
import {
  pauseAudio,
  playNextTrack,
  rewindorPlayPrevTrack,
  seekAudio,
  toggleAudioPlaying,
} from './global/player'
import type { NavigationKeyName } from './navigable'
import { isKeyOf } from './utils/common'

let freezeInput = false

export function setInputFrozen(frozen: boolean) {
  freezeInput = frozen
}

export function setupInputHandler() {
  document.body.addEventListener('keydown', (e) => {
    // showNotification({ title: 'Key Down', message: `Key "${e.key}" pressed.`, type: 'info' })

    if (e.ctrlKey || e.altKey || e.shiftKey || e.metaKey || e.key.match(/^F5|F8|F9|F1\d$/)) {
      return
    }

    if (
      // oxlint-disable-next-line typescript/prefer-optional-chain (buggy)
      e.target instanceof Element &&
      e.target.tagName.toLocaleLowerCase() === 'input' &&
      !(e.key.startsWith('Arrow') || e.key.match(/^F\d+/))
    ) {
      return
    }

    // TODO: improve by establishing a long with the existing short-press and long-press key tables
    if (freezeInput && e.key !== 'F5' && e.key !== 'Escape' && e.key !== 'F1') {
      e.preventDefault()
      return
    }

    e.preventDefault()

    const { key } = e

    if (isKeyOf(LONG_KEYS_HANDLER, key)) {
      const longPressData = longPressKeys.get(key)

      if (!longPressData) {
        const newData: LongPressData = {
          started: new Date(),
          timeout: setTimeout(() => {
            newData.alreadyTriggered = true
            LONG_KEYS_HANDLER[key].long()
          }, KEY_LONG_PRESS_THRESHOLD_MS),
          alreadyTriggered: false,
        }

        longPressKeys.set(key, newData)
      }
    } else if (Object.hasOwn(keysMeaning, e.key)) {
      navigationManager.dispatchKeyPress(keysMeaning[e.key])
    }
  })

  document.body.addEventListener('keyup', (e) => {
    // showNotification({ title: 'Key Up', message: `Key "${e.key}" pressed.`, type: 'info' })

    if (e.ctrlKey || e.altKey || e.shiftKey || e.metaKey) {
      return
    }

    const { key } = e

    if (!isKeyOf(LONG_KEYS_HANDLER, key)) {
      return
    }

    const longPressData = longPressKeys.get(key)

    if (!longPressData) {
      return
    }

    e.preventDefault()

    longPressKeys.delete(key)

    if (!longPressData.alreadyTriggered) {
      clearTimeout(longPressData.timeout)
      LONG_KEYS_HANDLER[key].short()
    }
  })

  // Pause audio when the page is hidden
  //
  // On Android TV especially, the sound instantnly becomes distorted
  // when the app is put in the background
  document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'hidden') {
      pauseAudio()
    }
  })
}

const keysMeaning: Record<string, NavigationKeyName> = {
  ArrowUp: 'UP',
  ArrowLeft: 'LEFT',
  ArrowRight: 'RIGHT',
  ArrowDown: 'DOWN',
  Escape: 'BACK',

  // Translated keys from custom Android TV app (see `CustomWebClient.kl`)
  F1: 'BACK',
  // F2 is 'SHORT_PRESS' / 'LONG_PRESS' (see below)
  F3: 'UP',
  F4: 'DOWN',
  // F5 is reserved (page refresh)
  F6: 'LEFT',
  F7: 'RIGHT',
}

type LongPressData = { started: Date; timeout: number; alreadyTriggered: boolean }

const longPressKeys = new Map<keyof typeof LONG_KEYS_HANDLER, LongPressData>()

const LONG_KEYS_HANDLER = {
  Enter: {
    short: () => navigationManager.dispatchKeyPress('SHORT_PRESS'),
    long: () => navigationManager.dispatchKeyPress('LONG_PRESS'),
  },

  MediaPlayPause: {
    short: toggleAudioPlaying,
    long: toggleBlackOutModeFeature,
  },

  MediaRewind: {
    short: () => seekAudio(-10),
    long: rewindorPlayPrevTrack,
  },

  MediaFastForward: {
    short: () => seekAudio(10),
    long: playNextTrack,
  },

  // Translated keys from custom Android TV app (see `CustomWebClient.kl`)
  F2: {
    short: () => navigationManager.dispatchKeyPress('SHORT_PRESS'),
    long: () => navigationManager.dispatchKeyPress('LONG_PRESS'),
  },
} satisfies Record<string, { short: () => void; long: () => void }>

const KEY_LONG_PRESS_THRESHOLD_MS = 250
