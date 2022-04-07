import { logError } from '../stores/debugger'

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

const pendingKeyCodes: Record<string, Date> = {}

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

  pendingKeyCodes[e.key] = new Date()

  e.preventDefault()
  return false
})

document.body.addEventListener('keyup', (e) => {
  if (e.ctrlKey || e.shiftKey || e.altKey) {
    return
  }

  if (!Object.prototype.hasOwnProperty.call(pendingKeyCodes, e.key)) {
    return logError('Got "keyup" event for a key without an associated "keydown" registration')
  }

  dispatchKeyPress(e.key, Date.now() > pendingKeyCodes[e.key].getTime() + LONG_PRESS_THRESOLD_MS)
  delete pendingKeyCodes[e.key]

  e.preventDefault()
  return false
})
