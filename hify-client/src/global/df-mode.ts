import { useEffect } from 'react'
import { useResettableTimeout, useOnUnmounted, useValueWatcher } from '#/utils/hooks.ts'
import { createGlobalStore, useGlobalStore } from '#/utils/stores.ts'
import { audioStateStore } from './player'

export const isDfModeFeatureEnabledStore = createGlobalStore(true)
export const isDfModeActiveStore = createGlobalStore(false)

const DISTRACTION_FREE_DELAY_MS = 3000

// Keys that shouldn't exit distraction-free mode when pressed
const DF_FREE_KEYS = new Set(['MediaPlayPause', 'MediaRewind', 'MediaFastForward'])

export function useDistractionFree(enabledLocally: boolean) {
  const isFeatureEnabled = useGlobalStore(isDfModeFeatureEnabledStore)
  const isDfActive = useGlobalStore(isDfModeActiveStore)
  const audioState = useGlobalStore(audioStateStore)

  const enabled = isFeatureEnabled && enabledLocally

  const dfModeTimeout = useResettableTimeout(() => {
    isDfModeActiveStore.mutate(true)
  }, DISTRACTION_FREE_DELAY_MS)

  useEffect(() => {
    if (audioState === 'playing' && enabled) {
      dfModeTimeout.restart()
    }
  }, [audioState, enabled, dfModeTimeout])

  useValueWatcher(
    isFeatureEnabled,
    (featureEnabled) => {
      if (!featureEnabled) {
        dfModeTimeout.clear()
        isDfModeActiveStore.mutate(false)
      } else if (audioState === 'playing' && enabled) {
        dfModeTimeout.restart()
      }
    },
    { immediate: true },
  )

  const onUserActivity = (e: Event) => {
    if (isDfActive) {
      e.stopImmediatePropagation()
      isDfModeActiveStore.mutate(false)
    }

    dfModeTimeout.restart()
  }

  const onKeyPress = (e: KeyboardEvent) => {
    if (
      enabled &&
      !(e.shiftKey || e.ctrlKey || e.altKey || e.metaKey) &&
      !DF_FREE_KEYS.has(e.key)
    ) {
      onUserActivity(e)
    }
  }

  const onMouseMove = (e: MouseEvent) => {
    if (enabled) {
      onUserActivity(e)
    }
  }

  useEffect(() => {
    window.addEventListener('keydown', onKeyPress, true)
    window.addEventListener('mousemove', onMouseMove, true)

    return () => {
      window.removeEventListener('keydown', onKeyPress, true)
      window.removeEventListener('mousemove', onMouseMove, true)
    }
  }, [onKeyPress, onMouseMove])

  useOnUnmounted(() => {
    dfModeTimeout.clear()
    isDfModeActiveStore.mutate(false)
  })

  return isDfActive
}
