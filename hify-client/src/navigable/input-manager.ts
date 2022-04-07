export function handleInput(handler: InputHandler): void {
  inputHandlers.push(handler)
}

export function dispatchKeyPress(key: KeyboardEvent['key'], long: boolean) {
  for (const handler of inputHandlers) {
    if (handler(key, long) === false) {
      return
    }
  }
}

export const LONG_PRESS_THRESOLD_MS = 500

export type InputHandler = (key: KeyboardEvent['key'], long: boolean) => boolean | void

const inputHandlers: InputHandler[] = [
  (key, long) => {
    console.debug(`${key} (${long ? 'Long' : 'Short'})`)
  },
]

const pendingKeyCodes: Record<string, RegisteredKeyPress> = {}

type RegisteredKeyPress = {
  at: Date
  timeout: number
}

export enum KeyPressType {
  Simple,
  Long,
}

document.body.addEventListener('keydown', (e) => {
  if (e.ctrlKey || e.shiftKey || e.altKey) {
    return
  }

  // Holding a key down will fire a repeated series of 'keydown' events, so we take care of ignoring them
  if (Object.prototype.hasOwnProperty.call(pendingKeyCodes, e.key)) {
    return
  }

  pendingKeyCodes[e.key] = {
    at: new Date(),
    timeout: window.setTimeout(() => {
      dispatchKeyPress(e.key, true)
      delete pendingKeyCodes[e.key]
    }, LONG_PRESS_THRESOLD_MS),
  }

  e.preventDefault()
  return false
})

document.body.addEventListener('keyup', (e) => {
  if (e.ctrlKey || e.shiftKey || e.altKey) {
    return
  }

  // Happens when key has been pressed long enough for a long press
  if (!Object.prototype.hasOwnProperty.call(pendingKeyCodes, e.key)) {
    return
  }

  clearTimeout(pendingKeyCodes[e.key].timeout)

  dispatchKeyPress(e.key, false)
  delete pendingKeyCodes[e.key]

  e.preventDefault()
  return false
})
