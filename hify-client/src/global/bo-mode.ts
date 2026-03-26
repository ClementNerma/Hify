import { createGlobalStore } from '#/utils/stores.ts'
import { showNotification } from './notifications'

export const isBlackOutModeFeatureEnabledStore = createGlobalStore(true)

export function toggleBlackOutModeFeature() {
  isBlackOutModeFeatureEnabledStore.mutateWith((enabled) => {
    showNotification({
      type: 'info',
      title: 'Black Out Mode',
      message: `Black Out Mode is now ${enabled ? 'disabled' : 'enabled'}.`,
      durationMs: 2000,
      hideProgressBar: true,
    })

    return !enabled
  })
}
