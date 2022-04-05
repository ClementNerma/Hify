import { derived, writable } from 'svelte/store'

type LogEntry = {
  at: Date
  level: LogLevel
  message: string
}

export enum LogLevel {
  Verbose = 'verbose',
  Info = 'info',
  Warn = 'warn',
  Error = 'error',
}

const logs = writable<LogEntry[]>([])

export const appLogs = derived(logs, (_) => [..._].reverse())

const consoleMethods: { [level in LogLevel]: 'log' | 'info' | 'warn' | 'error' } = {
  [LogLevel.Verbose]: 'log',
  [LogLevel.Info]: 'info',
  [LogLevel.Warn]: 'warn',
  [LogLevel.Error]: 'error',
}

export function log(level: LogLevel, message: string): void {
  logs.update((logs) => [...logs, { at: new Date(), level, message }])
  console[consoleMethods[level]](message)
}

export const logVerbose = (message: string) => log(LogLevel.Verbose, message)
export const logInfo = (message: string) => log(LogLevel.Info, message)
export const logWarn = (message: string) => log(LogLevel.Warn, message)
export const logError = (message: string) => log(LogLevel.Error, message)
