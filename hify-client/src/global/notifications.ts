import { randomId } from '#/utils/common.ts'
import { createGlobalStore } from '#/utils/stores.ts'

export type Notification = {
  id: string
  type: NotificationType
  title: string
  message: string
  durationMs?: number
  hideProgressBar?: boolean
}

export type NotificationType = 'error' | 'warning' | 'info'

export const notificationStore = createGlobalStore<Notification[]>([])

export function showNotification(params: Omit<Notification, 'id'>): void {
  notificationStore.mutateWith((notifs) => [...notifs, { id: randomId(), ...params }])
}

export function showQuickNotification(
  params: Omit<Notification, 'id' | 'durationMs' | 'hideProgressBar'>,
): void {
  showNotification({ ...params, durationMs: 3000, hideProgressBar: true })
}

export function showFailure(message: string, type: NotificationType = 'error'): void {
  showNotification({ type, title: 'Internal Error', message })
}
