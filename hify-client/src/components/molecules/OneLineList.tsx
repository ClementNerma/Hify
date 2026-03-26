import { useState, type ReactNode } from 'react'
import { TbCaretDownFilled, TbCaretUpDownFilled, TbCaretUpFilled } from 'react-icons/tb'
import { useValuesWatcher } from '#/utils/hooks.ts'
import { NavItem, type NavItemProps } from '../navigables/Item'

export type OneLineListProps<T extends boolean | number | string> = {
  items: OneLineListItem<T>[]
  onSelect: (item: T) => void
} & Omit<NavItemProps, `on${string}`>

export type OneLineListItem<T> = {
  label: ReactNode
  value: T
}

export function OneLineList<T extends boolean | number | string>({
  items,
  onSelect,
  ...itemProps
}: OneLineListProps<T>) {
  const [selected, setSelected] = useState(0)

  useValuesWatcher(
    items.map((item) => item.value),
    () => {
      setSelected(0)
    },
  )

  if (items.length === 0) {
    // showFailure('OneLineList: no items to display')
    return null
  }

  const isFirst = selected === 0
  const isLast = selected + 1 === items.length

  return (
    <NavItem
      {...itemProps}
      onPress={() => onSelect(items[selected].value)}
      // yoh
      onUpKey={() => {
        if (isFirst) {
          return { type: 'propagate' }
        }

        setSelected(selected - 1)
      }}
      onDownKey={() => {
        if (isLast) {
          return { type: 'propagate' }
        }

        setSelected(selected + 1)
      }}
    >
      {isFirst && !isLast && <TbCaretDownFilled />}
      {!isFirst && isLast && <TbCaretUpFilled />}
      {!isFirst && !isLast && <TbCaretUpDownFilled />} {items[selected].label}
    </NavItem>
  )
}
