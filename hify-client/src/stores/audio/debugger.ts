import { derived, writable } from 'svelte/store'

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

const logs = writable<LogEntry[]>([])

export const appLogs = derived(logs, (_) => [..._].reverse())

export function log(level: LogLevel, message: string): void {
  logs.update((logs) => [...logs, { at: new Date(), level, message }])

  const typedLevel: keyof typeof console = level
  console[typedLevel](message)
}

export const logDebug = (message: string) => log(LogLevel.Debug, message)
export const logInfo = (message: string) => log(LogLevel.Info, message)
export const logWarn = (message: string) => log(LogLevel.Warn, message)
export const logError = (message: string) => log(LogLevel.Error, message)
