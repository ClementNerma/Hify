import { logFatal } from '@/navigable'
import { readonly, ref } from 'vue'

export enum NotificationLevel {
	Info = 'INFO',
	Warn = 'WARN',
	Error = 'ERROR',
}

type Notification = {
	id: string
	level: NotificationLevel
	message: string
	timeout: number
}

const notifications = ref<Notification[]>([])

export const pendingNotifications = readonly(notifications)

export function showNotification(level: NotificationLevel, message: string, durationSecs?: number): string {
	const id = Math.floor(Math.random() * 1_000_000_000_000_000).toString()

	notifications.value.push({
		id,
		level,
		message,
		timeout: window.setTimeout(
			() => {
				removeNotification(id)
			},
			(durationSecs ?? 5) * 1000,
		),
	})

	return id
}

export function removeNotification(id: string): void {
	const notifIndex = notifications.value.findIndex((notif) => notif.id === id)

	if (notifIndex === -1) {
		logFatal(`Cannot remove non-existing notification "${id}"`)
	}

	clearTimeout(notifications.value[notifIndex].timeout)
	notifications.value.splice(notifIndex, 1)
}
