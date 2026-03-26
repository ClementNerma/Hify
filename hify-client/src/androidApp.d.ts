/**
 * @file Module used for integration with the Android TV application
 */

type HifyAndroidInjectedObject = {
  updateAppUrl(): void
}

/**
 * Integration API for the Android TV application
 */
export const hifyInterface =
  'hifyAndroidInjectedObject' in window
    ? // oxlint-disable-next-line typescript/no-unsafe-type-assertion
      (window.hifyAndroidInjectedObject as HifyAndroidInjectedObject)
    : null
