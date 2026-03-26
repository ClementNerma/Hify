import type { PropsWithChildren } from 'react'
import { NavItem, type NavItemProps } from '../navigables/Item'

export type CheckBoxProps = {
  checked: boolean
  onChange: (checked: boolean) => void
} & Omit<NavItemProps, 'onPress'>

export function CheckBox({
  checked,
  onChange,
  children,
  ...navProps
}: PropsWithChildren<CheckBoxProps>) {
  return (
    <NavItem onPress={() => onChange(!checked)} {...navProps}>
      {/* oxlint-disable-next-line react/checked-requires-onchange-or-readonly */}
      <input type="checkbox" checked={checked} /> {children}
    </NavItem>
  )
}
