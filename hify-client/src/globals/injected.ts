declare const hifyAndroidInjectedObject: {
  updateAppUrl(): void
}

export const hifyInterface = 'hifyAndroidInjectedObject' in window ? hifyAndroidInjectedObject : null
