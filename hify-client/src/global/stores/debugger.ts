import type { LogLevel } from '@/navigable'
import { computed, reactive } from 'vue'

type LogEntry = {
	at: Date
	level: LogLevel
	message: string
}

const logs = reactive<LogEntry[]>([])

export const appLogs = computed(() => [...logs].reverse())

export function log(level: LogLevel, message: string, error?: unknown): void {
	// biome-ignore lint/style/noParameterAssign: <explanation>
	message = `${message}${
		error !== null && error !== undefined
			? ` |> ${error instanceof Error ? error.message : typeof error === 'string' ? error : '<unknown error>'}`
			: ''
	}`

	logs.push({ at: new Date(), level, message })

	CONSOLE_METHODS[level](message)
}

const CONSOLE_METHODS: Record<LogLevel, (message: string) => void> = {
	Debug: console.debug,
	Info: console.info,
	Warn: console.warn,
	Error: console.error,
	Fatal: console.error,
}
