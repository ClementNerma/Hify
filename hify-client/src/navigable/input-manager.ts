export function handleInput(handler: InputHandler): void {
  inputHandlers.push(handler)
}

export function dispatchKeyPress(key: KeyboardEvent['key'], pressType: KeyPressType) {
  for (const handler of inputHandlers) {
    if (handler(key, pressType) === false) {
      return
    }
  }
}

export const DOUBLE_PRESS_THRESOLD_MS = 200
export const LONG_PRESS_THRESOLD_MS = 500

export type InputHandler = (key: KeyboardEvent['key'], pressType: KeyPressType) => boolean | void

const inputHandlers: InputHandler[] = [
  (key, type) => {
    console.log(`${key} (${type === KeyPressType.Simple ? 'Simple' : type === KeyPressType.Long ? 'Long' : 'Double'})`)
  },
]

const pendingKeyCodes: Record<string, RegisteredKeyPress> = {}

export enum KeyPressType {
  Simple,
  Double,
  Long,
}

type RegisteredKeyPress = {
  at: Date
  wasReleasedOnce: boolean
  timeout: number
}

document.body.addEventListener('keydown', (e) => {
  if (e.ctrlKey || e.shiftKey || e.altKey) {
    return
  }

  if (Object.prototype.hasOwnProperty.call(pendingKeyCodes, e.key)) {
    if (pendingKeyCodes[e.key].wasReleasedOnce) {
      dispatchKeyPress(e.key, KeyPressType.Double)
      delete pendingKeyCodes[e.key]
    }
  } else {
    pendingKeyCodes[e.key] = {
      at: new Date(),
      wasReleasedOnce: false,
      timeout: window.setTimeout(() => {
        if (Object.prototype.hasOwnProperty.call(pendingKeyCodes, e.key) && pendingKeyCodes[e.key].wasReleasedOnce) {
          delete pendingKeyCodes[e.key]
          dispatchKeyPress(e.key, KeyPressType.Simple)
        }
      }, DOUBLE_PRESS_THRESOLD_MS),
    }
  }

  e.preventDefault()
  return false
})

document.body.addEventListener('keyup', (e) => {
  if (e.ctrlKey || e.shiftKey || e.altKey) {
    return
  }

  if (!Object.prototype.hasOwnProperty.call(pendingKeyCodes, e.key)) {
    return
  }

  const pending = pendingKeyCodes[e.key]

  const firstPressedAt = pending.at.getTime()
  const now = Date.now()

  let pressType: KeyPressType | null

  if (now > firstPressedAt + LONG_PRESS_THRESOLD_MS) {
    pressType = KeyPressType.Long
  } else if (now > firstPressedAt + DOUBLE_PRESS_THRESOLD_MS) {
    pressType = KeyPressType.Simple
  } else {
    pressType = null
  }

  if (pressType !== null) {
    clearTimeout(pending.timeout)
    dispatchKeyPress(e.key, pressType)
    delete pendingKeyCodes[e.key]
  } else {
    pendingKeyCodes[e.key].wasReleasedOnce = true
  }
})
