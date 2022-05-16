import { onDestroy } from 'svelte'
import { writable } from 'svelte/store'
import { handleInput } from '../navigable/input-manager'

export const DISTRACTION_FREE_CLASSNAME = 'distraction-free'

export const distractionFreeMode = writable(false)

distractionFreeMode.subscribe((set) => {
  if (set) {
    document.body.classList.add(DISTRACTION_FREE_CLASSNAME)
  } else {
    document.body.classList.remove(DISTRACTION_FREE_CLASSNAME)
  }
})

export function setupDistractionFreeListener(delay: number, ignoreKeys?: string[], interceptTurningOn?: () => boolean) {
  function startDistractionFreeTimeout(): number | null {
    return window.setTimeout(() => {
      if (interceptTurningOn?.() !== false) {
        distractionFreeMode.set(true)
      }
    }, delay)
  }

  function resetDistractionFreeMode(stop = false): void {
    if (distractionModeTimeout !== null) {
      window.clearTimeout(distractionModeTimeout)
    }

    if (!stop) {
      distractionFreeMode.set(false)
      distractionModeTimeout = startDistractionFreeTimeout()
    }
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

  let distractionModeTimeout = startDistractionFreeTimeout()

  onDestroy(() => {
    destroyed = true
    resetDistractionFreeMode(true)
  })
}
