import type { PropsWithChildren } from 'react'
import { openContextMenu } from '#/global/ctx-menu.tsx'
import { Button } from '../atoms/Button'
import type { ContextMenuItem } from './ContextMenu'

export type DropDownProps = PropsWithChildren<{ menuItems: ContextMenuItem[] }>

export function DropDown({ children, menuItems }: DropDownProps) {
  return <Button onPress={() => openContextMenu(menuItems)}>{children}</Button>
}
