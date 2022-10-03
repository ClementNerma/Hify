import { writable } from 'svelte/store'

export const customBgColor = writable<[r: number, g: number, b: number, a?: number] | null>(null)

customBgColor.subscribe((value) => {
  if (value === null) {
    document.body.classList.remove('custom-bg-color')
    document.body.removeAttribute('style')
  } else {
    const [r, g, b, a] = value

    document.body.classList.add('custom-bg-color')
    document.body.setAttribute('style', `--linear-start: rgba(${r}, ${g}, ${b}, ${a ?? 1}); --linear-end: #000;`)
  }
})
