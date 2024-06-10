// TODO: move this file?

import { computed, readonly, ref } from 'vue'

const _errors = ref<ErrorDialogContent[]>([])

export const errors = readonly(_errors)

export const currentError = computed(() => (_errors.value.length > 0 ? _errors.value[0] : null))

export type ErrorDialogContent = {
	title: string
	message: string
	details: string | null
}

export function showErrorDialog(title: string, message: string, details?: string | null) {
	_errors.value = [
		..._errors.value,
		{
			title,
			message,
			details: details ?? null,
		},
	]
}

export function popError() {
	_errors.value = _errors.value.slice(1)
}
