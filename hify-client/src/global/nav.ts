import {
  NavigationManager,
  type RegistryItem,
  type RegistryItemProps,
  type UntypedNavigablesSet,
} from '../navigable'
import { NavigableGridHandler } from '../navigable/elements/grid'
import { NavigableItemHandler } from '../navigable/elements/item'
import { NavigableListHandler } from '../navigable/elements/list'
import { NavigableRowHandler } from '../navigable/elements/row'

const navigableHandlers = {
  [NavigableListHandler.navType]: NavigableListHandler,
  [NavigableRowHandler.navType]: NavigableRowHandler,
  [NavigableGridHandler.navType]: NavigableGridHandler,
  [NavigableItemHandler.navType]: NavigableItemHandler,
} satisfies UntypedNavigablesSet

// TODO: move query selector outside of this file
export const navigationManager = new NavigationManager(
  // oxlint-disable-next-line typescript/no-non-null-assertion
  document.querySelector('#root')!,
  navigableHandlers,
)

export type NavigableHandlers =
  typeof navigationManager extends NavigationManager<infer R> ? R : never

export type NavRegistryItem<N extends keyof typeof navigableHandlers> = RegistryItem<
  NavigableHandlers,
  N
>

export type NavRegistryItemProps<N extends keyof NavigableHandlers> = RegistryItemProps<
  NavigableHandlers,
  N
>
