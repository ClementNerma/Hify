import { InputHandlingResult, handleInput, type KeyPress } from '@/navigable'
import { onUnmounted, readonly, ref } from 'vue'

const NEVER_WAKE_UP_FOR_KEYS = ['Control', 'Shift', 'Alt']

const _distractionFreeMode = ref(false)

export const distractionFreeMode = readonly(_distractionFreeMode)

export type DistractionFreeListenerOptions = {
	delayMillis: number
	dontWakeUpForKeys?: Partial<Omit<KeyPress, 'longPress'>>[]
	darkeningCondition?: () => boolean
}

export function setupDistractionFreeListener({
	delayMillis,
	dontWakeUpForKeys,
	darkeningCondition,
}: DistractionFreeListenerOptions): (enabled: boolean) => void {
	function restartDistractionFreeTimeout() {
		if (destroyed) {
			return
		}

		if (distractionModeTimeout !== null) {
			clearTimeout(distractionModeTimeout)
		}

		distractionModeTimeout = window.setTimeout(() => {
			if (darkeningCondition?.() !== false) {
				_distractionFreeMode.value = true
			}
		}, delayMillis)
	}

	function resetDistractionFreeMode(): void {
		if (destroyed) {
			return
		}

		_distractionFreeMode.value = false
		restartDistractionFreeTimeout()
	}

	function externallySetDistractionFreeMode(value: boolean): void {
		if (destroyed) {
			return
		}

		if (value) {
			_distractionFreeMode.value = true
		} else {
			resetDistractionFreeMode()
		}
	}

	let destroyed = false

	handleInput((key) => {
		if (
			!destroyed &&
			!dontWakeUpForKeys?.find(
				(c) =>
					c.key === key.key &&
					(c.ctrlKey === undefined || c.ctrlKey === key.ctrlKey) &&
					(c.altKey === undefined || c.altKey === key.altKey) &&
					(c.shiftKey === undefined || c.shiftKey === key.shiftKey),
			) &&
			!NEVER_WAKE_UP_FOR_KEYS.includes(key.key)
		) {
			if (_distractionFreeMode.value) {
				resetDistractionFreeMode()
				return InputHandlingResult.Intercepted
			}

			restartDistractionFreeTimeout()
		}

		return InputHandlingResult.Propagate
	})

	let distractionModeTimeout: number | null = null
	restartDistractionFreeTimeout()

	onUnmounted(() => {
		destroyed = true

		if (distractionModeTimeout !== null) {
			clearTimeout(distractionModeTimeout)
		}

		_distractionFreeMode.value = false
	})

	return externallySetDistractionFreeMode
}
