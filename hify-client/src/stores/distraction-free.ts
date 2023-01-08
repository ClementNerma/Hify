import { onDestroy } from 'svelte'
import { writable } from 'svelte/store'
import { readonly } from '../globals/utils'
import { handleInput } from '../navigable/input-manager'

const _distractionFreeMode = writable(false)

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
				_distractionFreeMode.set(true)
			}
		}, delay)
	}

	function resetDistractionFreeMode(): void {
		if (destroyed) {
			return
		}

		_distractionFreeMode.set(false)
		restartDistractionFreeTimeout()
	}

	function externallySetDistractionFreeMode(value: boolean): void {
		if (destroyed) {
			return
		}

		if (value) {
			_distractionFreeMode.set(true)
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

	onDestroy(() => {
		destroyed = true

		if (distractionModeTimeout !== null) {
			clearTimeout(distractionModeTimeout)
		}

		_distractionFreeMode.set(false)
		resetDistractionFreeMode()
	})

	return externallySetDistractionFreeMode
}
