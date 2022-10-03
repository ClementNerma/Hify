import { writable } from 'svelte/store'

export const customBgColor = writable<string | null>(null)

customBgColor.subscribe((value) => {
  if (value === null) {
    document.body.classList.remove('custom-bg-color')
    document.body.removeAttribute('style')
  } else {
    document.body.classList.add('custom-bg-color')
    document.body.setAttribute('style', `--custom-bg-color: ${value};`)
  }
})
