import type { PropsWithChildren } from 'react'
import { navigationManager, type NavRegistryItemProps } from '#/global/nav.ts'
import { useNavigable } from './hooks'

export type NavRowProps = PropsWithChildren<
  NavRegistryItemProps<'row'> & { className?: string; fixedNavId?: string }
>

export function NavRow({ children, className, fixedNavId, ...navProps }: NavRowProps) {
  const { domProps } = useNavigable(navigationManager, 'row', navProps, fixedNavId)

  return (
    <div className={className} {...domProps}>
      {children}
    </div>
  )
}
