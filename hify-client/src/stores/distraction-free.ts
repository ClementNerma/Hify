import { onDestroy } from 'svelte'
import { derived, writable } from 'svelte/store'
import { handleInput } from '../navigable/input-manager'

const _distractionFreeMode = writable(false)

export const distractionFreeMode = derived(_distractionFreeMode, (_) => _)

export function setupDistractionFreeListener(
  delay: number,
  ignoreKeys?: string[],
  interceptTurningOn?: () => boolean,
): (enabled: boolean) => void {
  function restartDistractionFreeTimeout() {
    if (destroyed) {
      return
    }

    if (distractionModeTimeout !== null) {
      clearTimeout(distractionModeTimeout)
    }

    distractionModeTimeout = window.setTimeout(() => {
      if (interceptTurningOn?.() !== false) {
        _distractionFreeMode.set(true)
      }
    }, delay)
  }

  function resetDistractionFreeMode(): void {
    if (destroyed) {
      return
    }

    _distractionFreeMode.set(false)
    restartDistractionFreeTimeout()
  }

  function externallySetDistractionFreeMode(value: boolean): void {
    if (destroyed) {
      return
    }

    if (!value) {
      resetDistractionFreeMode()
      return
    }

    restartDistractionFreeTimeout()
  }

  let destroyed = false

  handleInput((key) => {
    if (destroyed) {
      return
    }

    if (!ignoreKeys || !ignoreKeys.includes(key)) {
      resetDistractionFreeMode()
    }
  })

  let distractionModeTimeout: number | null = null
  restartDistractionFreeTimeout()

  onDestroy(() => {
    destroyed = true
    _distractionFreeMode.set(false)
    resetDistractionFreeMode()
  })

  return externallySetDistractionFreeMode
}
