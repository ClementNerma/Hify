import { derived, readonly, writable } from 'svelte/store'

const _errors = writable<ErrorDialogContent[]>([])

export const errors = readonly(_errors)

export const currentError = derived(_errors, (err) => (err.length > 0 ? err[0] : null))

export type ErrorDialogContent = {
	title: string
	message: string
	details: string | null
}

export function showErrorDialog(title: string, message: string, details?: string | null) {
	_errors.update((list) => [
		...list,
		{
			title,
			message,
			details: details ?? null,
		},
	])
}

export function popError() {
	_errors.update((list) => list.slice(1))
}
