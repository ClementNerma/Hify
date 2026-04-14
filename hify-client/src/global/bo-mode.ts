import { createGlobalStore } from '#/utils/stores.ts'
import { showQuickNotification } from './notifications'

export const isBlackOutModeFeatureEnabledStore = createGlobalStore(true)

export function toggleBlackOutModeFeature() {
  isBlackOutModeFeatureEnabledStore.mutateWith((enabled) => {
    showQuickNotification({
      type: 'info',
      title: 'Black Out Mode',
      message: `Black Out Mode is now ${enabled ? 'disabled' : 'enabled'}.`,
    })

    return !enabled
  })
}
