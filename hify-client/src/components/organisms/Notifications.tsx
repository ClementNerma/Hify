/**
 * @file Module written by Claude Opus 4.5 (Github Copilot); edited by hand
 */

import React, { useEffect, useState, useCallback } from 'react'
import { FaCircleInfo, FaTriangleExclamation, FaCircleXmark, FaXmark } from 'react-icons/fa6'
// oxlint-disable-next-line import/no-unassigned-import
import './Notifications.css'
import {
  notificationStore,
  type Notification,
  type NotificationType,
} from '#/global/notifications.ts'
import { useGlobalStore } from '#/utils/stores.ts'

// TODO: make notifications go up as soon as the previous one is dismissed
export function NotificationsContainer() {
  const notifications = useGlobalStore(notificationStore)

  return (
    <div
      className="fixed top-4 right-4 z-50 flex flex-col gap-3 pointer-events-none"
      aria-live="polite"
      aria-label="Notifications"
    >
      {notifications.map((notification) => (
        <NotificationItem
          key={notification.id}
          notification={notification}
          onRemove={(id) =>
            notificationStore.mutateWith((notifs) => notifs.filter((n) => n.id !== id))
          }
        />
      ))}
    </div>
  )
}

function NotificationItem({
  notification,
  onRemove,
}: {
  notification: Notification
  onRemove: (id: string) => void
}) {
  const [isExiting, setIsExiting] = useState(false)
  const [progress, setProgress] = useState(100)

  const config = notificationConfig[notification.type]

  const durationMs = notification.durationMs ?? DEFAULT_NOTIFICATION_DURATION_MS

  const handleRemove = useCallback(() => {
    setIsExiting(true)
    setTimeout(
      () => onRemove(notification.id),
      // Matches the exit animation's duration from the associated CSS file
      300,
    )
  }, [notification.id, onRemove])

  useEffect(() => {
    const startTime = Date.now()

    const updateProgress = () => {
      const elapsed = Date.now() - startTime
      const remaining = Math.max(0, 100 - (elapsed / durationMs) * 100)
      setProgress(remaining)

      if (remaining > 0) {
        requestAnimationFrame(updateProgress)
      } else {
        handleRemove()
      }
    }

    const animationId = requestAnimationFrame(updateProgress)
    return () => cancelAnimationFrame(animationId)
  }, [handleRemove, durationMs])

  return (
    <div
      className={`
        notification-item
        ${isExiting ? 'notification-exit' : 'notification-enter'}
        ${config.bgColor}
        relative overflow-hidden
        w-120 border rounded-lg shadow-lg
        pointer-events-auto
      `}
    >
      <div className="p-4">
        <div className="flex items-start gap-3">
          <span className={`shrink-0 mt-0.5 ${config.iconColor}`}>{config.icon}</span>
          <div className="flex-1 min-w-0">
            <p className="text-sm font-semibold text-gray-900">{notification.title}</p>
            <pre className="mt-1 text-sm text-gray-600 whitespace-pre-wrap">
              {notification.message}
            </pre>
          </div>
          <button
            onClick={handleRemove}
            className="shrink-0 p-1 rounded-md text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-colors"
            aria-label="Dismiss notification"
          >
            <FaXmark className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Progress bar container */}
      <div className="h-1 w-full bg-gray-200">
        {/* Progress bar - starts full, depletes from left */}
        {notification.hideProgressBar !== true && (
          <div
            className={`h-full ${config.progressColor} transition-none`}
            style={{
              width: `${progress}%`,
              marginLeft: 'auto',
            }}
          />
        )}
      </div>
    </div>
  )
}

const notificationConfig: Record<
  NotificationType,
  { icon: React.ReactNode; bgColor: string; progressColor: string; iconColor: string }
> = {
  error: {
    icon: <FaCircleXmark className="w-5 h-5" />,
    bgColor: 'bg-red-50 border-red-200',
    progressColor: 'bg-red-500',
    iconColor: 'text-red-500',
  },
  warning: {
    icon: <FaTriangleExclamation className="w-5 h-5" />,
    bgColor: 'bg-amber-50 border-amber-200',
    progressColor: 'bg-amber-500',
    iconColor: 'text-amber-500',
  },
  info: {
    icon: <FaCircleInfo className="w-5 h-5" />,
    bgColor: 'bg-blue-50 border-blue-200',
    progressColor: 'bg-blue-500',
    iconColor: 'text-blue-500',
  },
}

const DEFAULT_NOTIFICATION_DURATION_MS = 10_000
