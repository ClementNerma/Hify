import type { JSX, PropsWithChildren } from 'react'
import { navigationManager, type NavRegistryItemProps } from '#/global/nav.ts'
import { useOnMounted } from '#/utils/hooks.ts'
import { useNavigable } from './hooks'

export type NavItemProps = PropsWithChildren<
  NavRegistryItemProps<'item'> & {
    className?: string
    fixedNavId?: string | null
    onReady?: ((navId: string) => void) | null
    onMouseEnter?: (() => void) | null
    onMouseLeave?: (() => void) | null
    overrideOnClick?: (() => void) | null
  }
>

function InternalNavigableItem({
  children,
  tag: Tag,
  className,
  fixedNavId,
  onReady,
  onMouseEnter,
  onMouseLeave,
  overrideOnClick,
  ...navProps
}: NavItemProps & {
  tag: keyof JSX.IntrinsicElements
}) {
  const { navId, domProps } = useNavigable(
    navigationManager,
    'item',
    navProps,
    fixedNavId ?? undefined,
  )

  useOnMounted(() => {
    // Assertion: ensure nav ID + DOM is registered
    navigationManager.findDomById(navId)

    onReady?.(navId)
  })

  return (
    <Tag
      {...domProps}
      className={className}
      onClick={(e) => {
        e.preventDefault()

        if (overrideOnClick) {
          overrideOnClick()
          return
        }

        navigationManager.dispatchKeyPress('SHORT_PRESS')
      }}
      onContextMenu={(e) => {
        e.preventDefault()
        navigationManager.dispatchKeyPress('LONG_PRESS')
      }}
      onMouseEnter={() => {
        navigationManager.focusById(navId, null)
        onMouseEnter?.()
      }}
      onMouseLeave={() => {
        navigationManager.unfocus()
        onMouseLeave?.()
      }}
    >
      {children}
    </Tag>
  )
}

export function NavItem({ className, ...props }: Omit<NavItemProps, 'tag'>) {
  return (
    <InternalNavigableItem
      {...props}
      tag="span"
      className={className !== undefined ? `${className} inline-block` : 'inline-block'}
    />
  )
}

export function InlineNavItem(props: Omit<NavItemProps, 'tag'>) {
  return <InternalNavigableItem {...props} tag="span" />
}

export function BlockNavItem(props: Omit<NavItemProps, 'tag'>) {
  return <InternalNavigableItem {...props} tag="div" />
}

export function CustomTagNavItem<T extends keyof JSX.IntrinsicElements>(
  props: Omit<NavItemProps, 'tag'> & { tag: T },
) {
  return <InternalNavigableItem {...props} tag={props.tag} />
}
