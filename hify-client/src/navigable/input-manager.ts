import { logDebug } from '../stores/debugger'

export const REMAPPED_KEYS: Record<string, string> = {
	F4: 'Escape',
}

export function handleInput(handler: InputHandler): void {
	inputHandlers.push(handler)
}

export function dispatchKeyPress(key: KeyboardEvent['key'], long: boolean) {
	console.debug(`Pressed key: ${key} (${long ? 'long' : 'short'})`)

	if (Object.prototype.hasOwnProperty.call(REMAPPED_KEYS, key)) {
		key = REMAPPED_KEYS[key]
	}

	for (const handler of inputHandlers) {
		if (handler(key, long) === KeyPressHandling.Intercepted) {
			return
		}
	}
}

export function registerLongPressableKeys(...keys: string[]): void {
	for (const key of keys) {
		watchLongPressForKeys.add(key)
	}

	for (const [key, remapped] of Object.entries(REMAPPED_KEYS)) {
		if (keys.includes(key)) {
			watchLongPressForKeys.add(remapped)
		}
	}
}

export const LONG_PRESS_THRESOLD_MS = 250

export type InputHandler = (key: KeyboardEvent['key'], long: boolean) => KeyPressHandling | void

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

export enum KeyPressHandling {
	Intercepted,
	Propagate,
}

/** Determine if the key should NOT be intercepted */
function shouldNotInterceptKey(e: KeyboardEvent) {
	return e.ctrlKey || e.shiftKey || e.altKey
		// Allow to input normal characters
		|| e.key.match(/^[a-zA-Z0-9_\-\+\s]$/)
		// Allow to open developer tools
		|| e.key === 'F12'
}

document.body.addEventListener('keydown', (e) => {
	logDebug(`Key down: ${e.ctrlKey ? 'Ctrl + ' : ''}${e.altKey ? 'Alt + ' : ''}${e.shiftKey ? 'Shift + ' : ''}${e.key}`)

	if (shouldNotInterceptKey(e)) {
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
	logDebug(`Key up: ${e.ctrlKey ? 'Ctrl + ' : ''}${e.altKey ? 'Alt + ' : ''}${e.shiftKey ? 'Shift + ' : ''}${e.key}`)

	if (shouldNotInterceptKey(e)) {
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
