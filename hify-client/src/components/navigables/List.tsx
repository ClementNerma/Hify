import type { CSSProperties, PropsWithChildren } from 'react'
import { navigationManager, type NavRegistryItemProps } from '#/global/nav.ts'
import { useNavigable } from './hooks'

export type NavListProps = PropsWithChildren<
  NavRegistryItemProps<'list'> & {
    className?: string
    fixedNavId?: string
    style?: CSSProperties
    onMouseEnter?: () => void
    onMouseLeave?: () => void
  }
>

export function NavList({
  children,
  className,
  fixedNavId,
  style,
  onMouseEnter,
  onMouseLeave,
  ...navProps
}: NavListProps) {
  const { domProps } = useNavigable(navigationManager, 'list', navProps, fixedNavId)

  return (
    <div
      className={className}
      style={style}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      {...domProps}
    >
      {children}
    </div>
  )
}
