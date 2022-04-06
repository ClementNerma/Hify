import { getContext, setContext } from 'svelte'
import { get, writable } from 'svelte/store'
import { logWarn } from '../stores/audio/debugger'

export enum NavigationDirection {
  Up,
  Left,
  Right,
  Down,
}

export enum NavigationAction {
  Press,
  LongPress,
  Back,
}

export enum NavigationComingFrom {
  Below,
  Left,
  Right,
  Above,
}

export abstract class NavigableCommon {
  readonly identity: symbol
  readonly page: NavigablePage

  constructor(readonly parent: NavigableContainer) {
    this.identity = parent.identity
    this.page = parent.page
  }

  abstract navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem | null
  abstract navigateToLastItem(): NavigableItem | null

  abstract canHandleAction(key: NavigationAction): boolean
  abstract handleAction(key: NavigationAction): NavigableItem | null
}

export abstract class NavigableContainer extends NavigableCommon {
  readonly priorityFocusables: NavigableItem[] = []

  abstract append(navigable: Navigable): void
  abstract hasChild(child: Navigable): boolean
  abstract remove(child: Navigable): void
  abstract navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem | null

  registerPriorityFocus(item: NavigableItem): void {
    this.priorityFocusables.push(item)
  }

  unregisterPriorityFocus(item: NavigableItem): void {
    const index = this.priorityFocusables.indexOf(item)

    if (index === -1) {
      throw new Error('Tried to remove a focusable item which is not registered in this navigable container')
    }

    this.priorityFocusables.splice(index, 1)
  }

  getPriorityFocusItem(): NavigableItem | null {
    return this.priorityFocusables.find((item) => item.hasFocusPriority()) ?? null
  }

  canHandleAction(_: NavigationAction): boolean {
    return false
  }

  handleAction(_: NavigationAction): NavigableItem | null {
    throw new Error('This navigable container does not support actions')
  }
}

export abstract class NavigableArrayContainer extends NavigableContainer {
  abstract children(): Navigable[]

  navigateToLastItem(): NavigableItem | null {
    const children = this.children()

    for (let c = children.length - 1; c >= 0; c--) {
      const target = children[c].navigateToLastItem()

      if (target) {
        return target
      }
    }

    return null
  }
}

export abstract class NavigableItem extends NavigableCommon {
  abstract canHandleDirection(direction: NavigationDirection): boolean
  abstract handleDirection(direction: NavigationDirection): NavigableItem | null

  abstract underlyingElement(): HTMLNavigableItemWrapperElement

  abstract onFocus(): void
  abstract onUnfocus(): void

  abstract hasFocusPriority(): boolean | null

  navigateToFirstItemDown(_: NavigationComingFrom): NavigableItem {
    return this
  }

  navigateToLastItem(): NavigableItem | null {
    return this
  }

  scrollTo(): void {
    const el = this.underlyingElement()

    if (el.constructor.name !== HTMLNavigableItemWrapperElement.name) {
      throw new Error("Item's underlying element is not an " + HTMLNavigableItemWrapperElement.name)
    }

    el.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'nearest' })
  }
}

interface _NavigableContainerLike extends NavigableContainer {
  readonly identity: symbol

  // Required to ensure compatibility
  asContainer(): NavigableContainer
}

class NavigablePage implements _NavigableContainerLike {
  readonly identity = Symbol()
  readonly page: NavigablePage
  readonly priorityFocusables: NavigableItem[] = []

  private onlyChild: Navigable | null = null

  constructor(private readonly onRequestFocus: (item: NavigableItem) => void) {
    this.page = this
  }
  getPriorityFocusItem(): NavigableItem | null {
    throw new Error('Method not implemented.')
  }

  get parent(): NavigableContainer {
    throw new Error("Cannot access parent from the page's root component")
  }

  append(navigable: Navigable): void {
    if (this.onlyChild) {
      throw new Error('Pages can only contain a single root navigable')
    }

    this.onlyChild = navigable
  }

  hasChild(child: Navigable): boolean {
    return child === this.onlyChild
  }

  remove(child: Navigable): void {
    if (!this.onlyChild) {
      throw new Error('Cannot remove component from empty navigable page')
    }

    if (this.onlyChild !== child) {
      throw new Error("Tried to remove another navigable than the page's root one")
    }

    this.onlyChild = null
  }

  navigate(_: NavigableContainer, __: NavigationDirection): NavigableItem | null {
    return null
  }

  navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem | null {
    return this.onlyChild ? this.onlyChild.navigateToFirstItemDown(from) : null
  }

  navigateToLastItem(): NavigableItem | null {
    return this.onlyChild ? this.onlyChild.navigateToLastItem() : null
  }

  canHandleAction(_: NavigationAction): boolean {
    return false
  }

  handleAction(_: NavigationAction): NavigableItem | null {
    throw new Error('Tried to make the navigable page component handle an action')
  }

  registerPriorityFocus(_: NavigableItem): void {
    throw new Error('Cannot register an item as focusable with priority from the page itself')
  }

  unregisterPriorityFocus(_: NavigableItem): void {
    throw new Error('Cannot unregister a focusable with priority item from the page itself')
  }

  asContainer(): NavigableContainer {
    return this
  }

  requestFocus(item: NavigableItem): void {
    if (item.parent.page !== this) {
      throw new Error("Cannot request focus for an element that isn't part of the same page")
    }

    this.onRequestFocus(item)
  }
}

export function getParentNavigable(): NavigableContainer {
  const nav = getContext(NAVIGATION_CTX)

  if (nav === null || nav === undefined) {
    throw new Error('No parent navigable found in the current context')
  }

  if (!(nav instanceof NavigableContainer) && !(nav instanceof NavigablePage)) {
    throw new Error('Context does not contain a navigable value')
  }

  return nav
}

export function setChildrenNavigable(nav: NavigableContainer) {
  setContext(NAVIGATION_CTX, nav)
}

export function usePageNavigator(): NavigableContainer {
  const page = new NavigablePage(_requestFocus)

  navState.update((state) => {
    state?.focused?.onUnfocus()
    return { page, focused: null }
  })

  setChildrenNavigable(page)

  return page
}

export function wasNavigableDestroyed(navigable: Navigable): boolean {
  while (!(navigable instanceof NavigablePage)) {
    if (!navigable.parent.hasChild(navigable)) {
      return true
    }

    navigable = navigable.parent
  }

  return false
}

export function handleKeyboardEvent(e: KeyboardEvent): void | false {
  if (e.ctrlKey || e.shiftKey || e.altKey) {
    return
  }

  const state = get(navState)

  if (!state) {
    return
  }

  let __current = state.focused

  if (__current) {
    if (__current.identity !== state.page.identity) {
      logWarn('Previously-focused element has a different identity than the current page, removing focus')
      __current.onUnfocus()
      __current = null
    } else if (wasNavigableDestroyed(__current)) {
      logWarn('Previously-focused element was destroyed, removing focus')
      __current.onUnfocus()
      __current = null
    }
  }

  let currentJustFocused = false

  if (!__current) {
    __current = state.page.navigateToFirstItemDown(NavigationComingFrom.Above)

    if (!__current) {
      logWarn('No navigable item in this page')
      return
    }

    currentJustFocused = true
  }

  const current = __current

  let next: NavigableItem | null

  switch (e.key) {
    case 'ArrowUp':
    case 'ArrowLeft':
    case 'ArrowRight':
    case 'ArrowDown':
      if (currentJustFocused) {
        next = current
        break
      }

      const directions: { [key in typeof e.key]: NavigationDirection } = {
        ArrowUp: NavigationDirection.Up,
        ArrowLeft: NavigationDirection.Left,
        ArrowRight: NavigationDirection.Right,
        ArrowDown: NavigationDirection.Down,
      }

      const direction = directions[e.key]

      next = current.canHandleDirection(direction)
        ? current.handleDirection(direction)
        : current.parent.navigate(current, direction)

      break

    case 'Enter':
    case ' ':
    case 'Backspace':
    case 'Escape':
      const events: { [key in typeof e.key]: NavigationAction } = {
        Enter: NavigationAction.Press,
        ' ': NavigationAction.LongPress,
        Backspace: NavigationAction.Back,
        Escape: NavigationAction.Back,
      }

      const event = events[e.key]

      if (currentJustFocused && event !== NavigationAction.Back) {
        next = current
        break
      }

      next = null

      for (const nav of _getParentsWithItem(current)) {
        if (!nav.canHandleAction(event)) {
          continue
        }

        const newFocused = nav.handleAction(event)

        if (newFocused) {
          next = newFocused
          break
        }
      }

      break

    case 'Home':
      next = current.parent.navigateToFirstItemDown(NavigationComingFrom.Above)
      break

    case 'End':
      next = current.parent.navigateToLastItem()
      break

    default:
      return
  }

  if (next) {
    navState.set(_generateNavState(current, next, state.page))
  }

  e.preventDefault()
  return false
}

function _getParents(item: NavigableItem): NavigableContainer[] {
  const parents: NavigableContainer[] = []

  let current: NavigableContainer = item.parent

  while (!(current instanceof NavigablePage)) {
    parents.push(current)
    current = current.parent
  }

  return parents
}

function _getParentsWithItem(item: NavigableItem): Navigable[] {
  const out: Navigable[] = [item]
  return out.concat(_getParents(item))
}

function _checkItemValidity(item: NavigableItem, page: NavigablePage): boolean {
  if (item.identity !== page.identity) {
    logWarn('Previously-focused element has a different identity than the current page, removing focus')
    item.onUnfocus()
    return false
  }
  if (wasNavigableDestroyed(item)) {
    logWarn('Previously-focused element was destroyed, removing focus')
    item.onUnfocus()
    return false
  }

  return true
}

function _requestFocus(item: NavigableItem) {
  navState.update((state) =>
    state && _checkItemValidity(item, state.page) ? _generateNavState(state.focused, item, state.page) : state,
  )
}

function _generateNavState(oldFocused: NavigableItem | null, newFocused: NavigableItem, page: NavigablePage): NavState {
  oldFocused?.onUnfocus()
  newFocused.onFocus()

  newFocused.scrollTo()

  return { page, focused: newFocused }
}

export type Navigable = NavigableContainer | NavigableItem

const NAVIGATION_CTX = Symbol()

type NavState = {
  page: NavigablePage
  focused: NavigableItem | null
}

const navState = writable<NavState | null>(null)

export class HTMLNavigableItemWrapperElement extends HTMLElement {}

const itemWrapperInPlace = window.customElements.get('navigable-item-wrapper')

if (!itemWrapperInPlace) {
  window.customElements.define('navigable-item-wrapper', HTMLNavigableItemWrapperElement)
} else if (itemWrapperInPlace.name !== HTMLNavigableItemWrapperElement.name) {
  throw new Error('An invalid item wrapper element is already in place')
}
