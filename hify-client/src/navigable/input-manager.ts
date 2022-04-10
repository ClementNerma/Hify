import { logDebug } from '../stores/debugger'

export function handleInput(handler: InputHandler): void {
  inputHandlers.push(handler)
}

export function dispatchKeyPress(key: KeyboardEvent['key'], long: boolean) {
  logDebug(`Pressed key: ${key} (${long ? 'long' : 'short'})`)

  for (const handler of inputHandlers) {
    if (handler(key, long) === false) {
      return
    }
  }
}

export function registerLongPressableKey(key: string): void {
  watchLongPressForKeys.add(key)
}

export const LONG_PRESS_THRESOLD_MS = 250

export type InputHandler = (key: KeyboardEvent['key'], long: boolean) => boolean | void

const inputHandlers: InputHandler[] = []

const watchLongPressForKeys = new Set<string>()

const pendingKeyCodes: Record<string, RegisteredKeyPress> = {}
const triggeredKeyEvent = new Set<string>()

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

  e.preventDefault()
  e.stopImmediatePropagation()

  if (!watchLongPressForKeys.has(e.key)) {
    dispatchKeyPress(e.key, false)
    return false
  }

  // Holding a key down will fire a repeated series of 'keydown' events, so we take care of ignoring them
  if (Object.prototype.hasOwnProperty.call(pendingKeyCodes, e.key)) {
    return false
  }

  pendingKeyCodes[e.key] = {
    at: new Date(),
    timeout: window.setTimeout(() => {
      triggeredKeyEvent.add(e.key)
      dispatchKeyPress(e.key, true)
    }, LONG_PRESS_THRESOLD_MS),
  }

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

  if (!watchLongPressForKeys.has(e.key)) {
    return
  }

  if (triggeredKeyEvent.has(e.key)) {
    triggeredKeyEvent.delete(e.key)
    delete pendingKeyCodes[e.key]
    return
  }

  clearTimeout(pendingKeyCodes[e.key].timeout)

  dispatchKeyPress(e.key, false)
  delete pendingKeyCodes[e.key]
})
