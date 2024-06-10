import { handleInput } from '@/navigable/input-manager'
import { onUnmounted, readonly, ref } from 'vue'

const _distractionFreeMode = ref(false)

export const distractionFreeMode = readonly(_distractionFreeMode)

export function setupDistractionFreeListener(
	delay: number,
	ignoreKeys?: string[],
	interceptTurningOn?: () => boolean,
): (enabled: boolean) => void {
	function restartDistractionFreeTimeout() {
		if (destroyed) {
			return
		}

		if (distractionModeTimeout !== null) {
			clearTimeout(distractionModeTimeout)
		}

		distractionModeTimeout = window.setTimeout(() => {
			if (interceptTurningOn?.() !== false) {
				_distractionFreeMode.value = true
			}
		}, delay)
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
		if (destroyed) {
			return
		}

		if (!ignoreKeys?.includes(key)) {
			resetDistractionFreeMode()
		}
	})

	let distractionModeTimeout: number | null = null
	restartDistractionFreeTimeout()

	onUnmounted(() => {
		destroyed = true

		if (distractionModeTimeout !== null) {
			clearTimeout(distractionModeTimeout)
		}

		_distractionFreeMode.value = false
		resetDistractionFreeMode()
	})

	return externallySetDistractionFreeMode
}
