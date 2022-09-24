import { writable } from 'svelte/store'

export const BLACK_BACKGROUND_CLASS_NAME = 'black-background'

export const blackBackground = writable(false)

blackBackground.subscribe((enabled) => {
  if (enabled) {
    document.body.classList.add(BLACK_BACKGROUND_CLASS_NAME)
  } else {
    document.body.classList.remove(BLACK_BACKGROUND_CLASS_NAME)
  }
})
