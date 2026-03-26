import type { NavigableHandler } from '..'

// oxlint-disable-next-line typescript/ban-types, typescript/no-empty-object-type
type NavigableItemProps = {}

// oxlint-disable-next-line typescript/ban-types, typescript/no-empty-object-type
type NavigableItemState = {}

export const NavigableItemHandler: NavigableHandler<
  'item',
  NavigableItemProps,
  NavigableItemState
> = {
  navType: 'item',
  isContainer: false,

  createState() {
    return {}
  },

  enterFrom() {
    return { type: 'focusThis' }
  },

  navigate() {
    throw new Error('Cannot navigate inside items')
  },
}
