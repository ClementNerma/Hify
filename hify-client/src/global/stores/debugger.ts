import { computed, reactive } from 'vue'

type LogEntry = {
	at: Date
	level: LogLevel
	message: string
}

export enum LogLevel {
	Debug = 'debug',
	Info = 'info',
	Warn = 'warn',
	Error = 'error',
}

const logs = reactive<LogEntry[]>([])

export const appLogs = computed(() => [...logs].reverse())

export function log(level: LogLevel, message: string): void {
	logs.push({ at: new Date(), level, message })

	const typedLevel: keyof typeof console = level
	console[typedLevel](message)
}

export function logDebug(message: string) {
	log(LogLevel.Debug, message)
}

export function logInfo(message: string) {
	log(LogLevel.Info, message)
}

export function logWarn(message: string) {
	log(LogLevel.Warn, message)
}

export function logError(message: string, error?: unknown) {
	log(LogLevel.Error, error ? `${message} |> ${error instanceof Error ? error.message : '<unknown error>'}` : message)
}

export function logFatal(message: string, error?: unknown): never {
	logError(message, error)

	if (error) {
		console.error(error)
	}

	throw new Error(message)
}
