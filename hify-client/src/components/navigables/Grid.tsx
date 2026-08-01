import type { PropsWithChildren, ReactNode } from 'react'
import { navigationManager, type NavRegistryItemProps } from '#/global/nav.ts'
import { useNavigable } from './hooks'

export type RawNavGridProps = PropsWithChildren<
  NavRegistryItemProps<'grid'> & { className?: string }
>

export function RawNavGrid({ children, className, ...navProps }: RawNavGridProps) {
  const { domProps } = useNavigable(navigationManager, 'grid', navProps)

  return (
    <div {...domProps} className={className}>
      {children}
    </div>
  )
}

export type NavGridProps<T> = Omit<RawNavGridProps, 'children'> & {
  gapless?: boolean
  items: T[]
  keyOfItem: (item: T) => string
  children: (item: T, index: number) => ReactNode
}

export function NavGrid<T>({
  gapless,
  items,
  keyOfItem,
  children: renderItem,
  ...rest
}: NavGridProps<T>) {
  const { columns } = rest

  return (
    <RawNavGrid {...rest}>
      <div
        className={`grid ${TAILWIND_GRID_COLUMNS.get(columns)} auto-rows-fr ${gapless !== true ? 'gap-4' : ''}`}
      >
        {items.map((item, index) => (
          <div key={keyOfItem(item)} className="flex">
            {renderItem(item, index)}
          </div>
        ))}
      </div>
    </RawNavGrid>
  )
}

// HACK: required for Tailwind to properly generate classes in the CSS
const TAILWIND_GRID_COLUMNS = new Map<number, string>([
  [1, 'grid-cols-1'],
  [2, 'grid-cols-2'],
  [3, 'grid-cols-3'],
  [4, 'grid-cols-4'],
  [5, 'grid-cols-5'],
  [6, 'grid-cols-6'],
  [7, 'grid-cols-7'],
  [8, 'grid-cols-8'],
  [9, 'grid-cols-9'],
  [10, 'grid-cols-10'],
  [11, 'grid-cols-11'],
  [12, 'grid-cols-12'],
  [13, 'grid-cols-13'],
  [14, 'grid-cols-14'],
  [15, 'grid-cols-15'],
  [16, 'grid-cols-16'],
  [17, 'grid-cols-17'],
  [18, 'grid-cols-18'],
  [19, 'grid-cols-19'],
  [20, 'grid-cols-20'],
])
