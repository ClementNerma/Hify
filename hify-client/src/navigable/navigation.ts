import { getContext, setContext } from 'svelte'
import { writable } from 'svelte/store'

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

  abstract firstItemDown(from: NavigationComingFrom): NavigableItem | null

  abstract canHandleAction(key: NavigationAction): boolean
  abstract handleAction(key: NavigationAction): NavigableItem | null
}

export abstract class NavigableContainer extends NavigableCommon {
  abstract append(navigable: Navigable): void
  abstract hasChild(child: Navigable): boolean
  abstract remove(child: Navigable): void
  abstract navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem | null

  canHandleAction(_: NavigationAction): boolean {
    return false
  }

  handleAction(_: NavigationAction): NavigableItem | null {
    throw new Error('This navigable container does not support actions')
  }
}

export abstract class NavigableItem extends NavigableCommon {
  abstract canHandleDirection(direction: NavigationDirection): boolean
  abstract handleDirection(direction: NavigationDirection): NavigableItem | null

  abstract onFocus(): void
  abstract onUnfocus(): void

  firstItemDown(_: NavigationComingFrom): NavigableItem {
    return this
  }
}

class NavigablePage {
  readonly identity = Symbol()
  readonly page: NavigablePage
  private onlyChild: Navigable | null = null

  constructor() {
    this.page = this
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

  firstItemDown(from: NavigationComingFrom): NavigableItem | null {
    return this.onlyChild ? this.onlyChild.firstItemDown(from) : null
  }

  canHandleAction(_: NavigationAction): boolean {
    return false
  }

  handleAction(_: NavigationAction): NavigableItem | null {
    throw new Error('Tried to make the navigable page component handle an action')
  }

  // Required to ensure compatibility with the parent class even without inheritance
  asContainer(): NavigableContainer {
    return this
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
  const page = new NavigablePage()

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

export function handleKeyboardEvent(e: KeyboardEvent): void {
  if (e.ctrlKey || e.shiftKey || e.altKey) {
    return
  }

  navState.update((state) => {
    if (!state) {
      return state
    }

    let __current = state.focused

    if (__current) {
      if (__current.identity !== state.page.identity) {
        console.warn('Previously-focused element has a different identity than the current page, removing focus')
        __current = null
      } else if (wasNavigableDestroyed(__current)) {
        console.warn('Previously-focused element was destroyed, removing focus')
        __current = null
      }
    }

    const current = __current ?? state.page.firstItemDown(NavigationComingFrom.Above)

    if (!current) {
      console.warn('No navigable item in this page')
      return state
    }

    let next: NavigableItem | null

    switch (e.key) {
      case 'ArrowUp':
      case 'ArrowLeft':
      case 'ArrowRight':
      case 'ArrowDown':
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

      default:
        return state
    }

    if (!next) {
      return state
    }

    current.onUnfocus()
    next.onFocus()

    return { page: state.page, focused: next }
  })
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

export type Navigable = NavigableContainer | NavigableItem

const NAVIGATION_CTX = Symbol()

type NavState = {
  page: NavigablePage
  focused: NavigableItem | null
}

const navState = writable<NavState | null>(null)
