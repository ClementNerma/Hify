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

  constructor(readonly parent: NavigableContainer) {
    this.identity = parent.identity
  }

  abstract firstItemDown(from: NavigationComingFrom): NavigableItem | null

  abstract canHandleAction(key: NavigationAction): boolean
  abstract handleAction(key: NavigationAction): NavigableItem | null
}

export abstract class NavigableContainer extends NavigableCommon {
  abstract append(navigable: Navigable): void
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

class NavigablePage extends NavigableContainer {
  private onlyChild: Navigable | null = null

  constructor() {
    const fakeNav: Pick<NavigableContainer, 'identity'> = { identity: Symbol() }
    super(fakeNav as NavigableContainer)
  }

  append(navigable: Navigable): void {
    if (this.onlyChild) {
      throw new Error('Pages can only contain a single root navigable')
    }

    this.onlyChild = navigable
  }

  navigate(_: NavigableContainer, __: NavigationDirection): NavigableItem | null {
    return null
  }

  firstItemDown(from: NavigationComingFrom): NavigableItem | null {
    return this.onlyChild ? this.onlyChild.firstItemDown(from) : null
  }
}

export function getParentNavigable(): NavigableContainer {
  const nav = getContext(NAVIGATION_CTX)

  if (nav === null || nav === undefined) {
    throw new Error('No parent navigable found in the current context')
  }

  if (!(nav instanceof NavigableContainer)) {
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

  return page
}

function _getParents(item: NavigableItem): NavigableContainer[] {
  const parents: NavigableContainer[] = []

  let current: NavigableContainer = item.parent

  while (!(current.parent instanceof NavigablePage)) {
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

document.body.addEventListener('keydown', (e) => {
  if (e.ctrlKey || e.shiftKey || e.altKey) {
    return
  }

  navState.update((state) => {
    if (!state) {
      return state
    }

    const current = state.focused ?? state.page.firstItemDown(NavigationComingFrom.Above)

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
})
