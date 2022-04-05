import { derived, writable } from 'svelte/store'

type LogEntry = {
  at: Date
  message: string
}

const logs = writable<LogEntry[]>([])

export const appLogs = derived(logs, (_) => [..._].reverse())

export function log(message: string): void {
  logs.update((logs) => [...logs, { at: new Date(), message }])
}
