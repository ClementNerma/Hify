import type { PropsWithChildren } from 'react'
import { isDfModeActiveStore } from '#/global/df-mode.ts'
import { useGlobalStore } from '#/utils/stores.ts'

function DfToggle({ children, whenDf }: PropsWithChildren<{ whenDf: 'show' | 'hide' }>) {
  const dfActive = useGlobalStore(isDfModeActiveStore)

  return (
    <div
      className={`transition ${dfActive || whenDf === 'show' ? 'opacity-0 duration-700' : 'duration-500'}`}
    >
      {children}
    </div>
  )
}

export function DfFadeIn({ children }: PropsWithChildren) {
  return <DfToggle whenDf="show">{children}</DfToggle>
}

export function DfFadeOut({ children }: PropsWithChildren) {
  return <DfToggle whenDf="hide">{children}</DfToggle>
}

export function DfShow({ children }: PropsWithChildren) {
  const dfActive = useGlobalStore(isDfModeActiveStore)

  return dfActive ? children : null
}

export function DfHide({ children }: PropsWithChildren) {
  const dfActive = useGlobalStore(isDfModeActiveStore)

  return dfActive ? null : children
}
