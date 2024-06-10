/**
 * @file Module used for integration with the Android TV application
 */

declare const hifyAndroidInjectedObject: {
	updateAppUrl(): void
}

/**
 * Integration API for the Android TV application
 */
export const hifyInterface = 'hifyAndroidInjectedObject' in window ? hifyAndroidInjectedObject : null
