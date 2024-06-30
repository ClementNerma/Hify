import { InputHandlingResult, handleInput } from '@/navigable'
import { onUnmounted, readonly, ref } from 'vue'

const _distractionFreeMode = ref(false)

export const distractionFreeMode = readonly(_distractionFreeMode)

export type DistractionFreeListenerOptions = {
	delayMillis: number
	dontWakeUpForKeys?: string[]
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
		if (!destroyed && !dontWakeUpForKeys?.includes(key)) {
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
