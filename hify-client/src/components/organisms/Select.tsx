import { useState, type ReactNode } from 'react'
import { assert } from '#/utils/common.ts'
import type { ContextMenuItem } from './ContextMenu'
import { DropDown } from './DropDown'

export type SelectProps<T extends boolean | number | string> = {
  items: SelectItem<T>[]
  initialValue: T
  onSelect: (value: T) => void
}

export type SelectItem<T> = {
  icon: ReactNode
  label: string
  value: T
}

export function Select<T extends boolean | number | string>({
  items,
  initialValue,
  onSelect,
}: SelectProps<T>) {
  const initialIndex = items.findIndex((item) => item.value === initialValue)

  assert(initialIndex !== -1, '<Select />: Initial value must be in items')

  const [selected, setSelected] = useState(initialIndex)

  const menuItems = items.map(
    ({ icon, label }, i): ContextMenuItem => ({
      icon,
      label,
      onPress: () => {
        setSelected(i)
        onSelect(items[i].value)
      },
    }),
  )

  return (
    // TODO: focus priority on currently selected item
    <DropDown menuItems={menuItems}>
      {items[selected].icon} {items[selected].label}
    </DropDown>
  )
}

// export function TextSelect<T extends string>({
//   values,
//   onSelect,
// }: {
//   values: T[]
//   onSelect: (value: T) => void
// }) {
//   return (
//     <Select
//       items={values.map((item) => ({ icon: null, label: item, value: item }))}
//       onSelect={onSelect}
//     />
//   )
// }
