import type { PropsWithChildren } from 'react'
import { navigationManager, type NavRegistryItemProps } from '#/global/nav.ts'
import { useNavigable } from './hooks'

export type NavGridProps = PropsWithChildren<NavRegistryItemProps<'grid'> & { className?: string }>

export function NavGrid({ children, className, ...navProps }: NavGridProps) {
  const { domProps } = useNavigable(navigationManager, 'grid', navProps)

  return (
    <div {...domProps} className={className}>
      {children}
    </div>
  )
}
