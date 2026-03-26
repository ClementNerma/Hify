import type { PropsWithChildren } from 'react'
import { NavItem, type NavItemProps } from '#/components/navigables/Item.tsx'

export type ButtonProps = NavItemProps & { disabled?: boolean }

export function Button({
  children,
  disabled,
  ...navigableItemProps
}: PropsWithChildren<ButtonProps>) {
  return (
    <NavItem
      {...navigableItemProps}
      onPress={() => {
        if (disabled !== true) {
          navigableItemProps.onPress?.()
        }
      }}
      className={`px-2 py-1 ${disabled !== true ? 'bg-gray-600' : 'bg-gray-800'}`}
    >
      <button disabled={disabled}>{children}</button>
    </NavItem>
  )
}
