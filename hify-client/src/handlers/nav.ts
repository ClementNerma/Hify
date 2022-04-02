import { writable } from 'svelte/store'

type NavigationState = {
  pageNav: PageNavigator
  focusOn: NavigableItemProps | null
}

const navState = writable<NavigationState | null>(null)

export type NavigableItemProps = {
  parent: Navigator
  onFocusChange: (hasFocus: boolean) => void
  onPress: () => void
  onLongPress?: () => void
  onBack?: () => void
}

export type Navigable = { type: 'item'; props: NavigableItemProps } | { type: 'row'; nav: Navigator }

export class Navigator {
  readonly identity: symbol
  private readonly elements: Navigable[] = []

  constructor(readonly parent: Navigator | PageNavigator, private readonly onRegister: (navigable: Navigable) => void) {
    this.identity = parent.identity
  }

  register(navigable: Navigable): void {
    const childNav = navigable.type === 'item' ? navigable.props.parent : navigable.nav

    if (childNav.identity !== this.identity) {
      throw new Error('Identity mismatch in elements')
    }

    this.elements.push(navigable)
    this.onRegister(navigable)
  }

  getElements(): ReadonlyArray<Navigable> {
    return this.elements
  }

  getParentNavigator(): Navigator | null {
    return this.parent instanceof Navigator ? this.parent : null
  }

  createSubNavigator(): Navigator {
    return new Navigator(this, (navigable) => this.onRegister(navigable))
  }
}

export const NAVIGATION_CTX = Symbol()

export class PageNavigator {
  readonly identity = Symbol()
  readonly subNavigator: Navigator
  private hasDescendent = false

  constructor(readonly onFirstDescendent: (props: NavigableItemProps) => void, readonly onBack: () => void) {
    this.subNavigator = new Navigator(this, (navigable) => {
      if (!this.hasDescendent && navigable.type === 'item') {
        this.hasDescendent = true
        this.onFirstDescendent(navigable.props)
      }
    })
  }
}

export function usePageNavigator(onBack: () => void): Navigator {
  const pageNav = new PageNavigator(focusOn, onBack)

  // TODO: call unfocus on previous focusOn item?
  navState.set({
    pageNav,
    focusOn: null,
  })

  return pageNav.subNavigator
}

export function useNavigator(parent: Navigator): Navigator {
  return parent.createSubNavigator()
}

export function asNavigator(value: unknown): Navigator {
  if (value === null || value === undefined) {
    throw new Error('No navigator defined in the current context')
  }

  if (!(value instanceof Navigator)) {
    throw new Error('Provided value is not a navigator')
  }

  return value
}

export function focusOn(item: NavigableItemProps): void {
  navState.update((state) => {
    if (!state) {
      return state
    }

    if (item.parent.identity !== state.pageNav.identity) {
      throw new Error('Tried to focus on an element from another page')
    }

    state.focusOn?.onFocusChange(false)
    item.onFocusChange(true)

    return {
      pageNav: state.pageNav,
      focusOn: item,
    }
  })
}

function _findFirst(nav: Navigator): NavigableItemProps | null {
  for (const el of nav.getElements()) {
    if (el.type === 'item') {
      return el.props
    }

    const found = _findFirst(el.nav)

    if (found) {
      return found
    }
  }

  return null
}

function _findHorizontal(item: NavigableItemProps, direction: 'left' | 'right'): NavigableItemProps | null {
  const parentElements = item.parent.getElements()
  const selfIndex = parentElements.findIndex((el) => el.type === 'item' && el.props === item)

  if (selfIndex === -1) {
    throw new Error("Internal error: item not found in its parent's elements")
  }

  const newIndex = selfIndex + (direction === 'left' ? -1 : 1)

  if (newIndex === -1 || newIndex === parentElements.length) {
    return null
  }

  const newEl = parentElements[newIndex]

  if (newEl.type === 'row') {
    return null
  }

  return newEl.props
}

function _findVertical(item: NavigableItemProps, direction: 'up' | 'down'): NavigableItemProps | null {
  const elements = item.parent.getElements()
  let selfIndex = elements.findIndex((el) => el.type === 'item' && el.props === item)

  if (selfIndex === -1) {
    throw new Error("Internal error: item not found in its parent's elements")
  }

  let current = item.parent

  for (;;) {
    const sliced =
      direction === 'up'
        ? [...current.getElements().slice(0, selfIndex)].reverse()
        : current.getElements().slice(selfIndex + 1)

    for (const el of sliced) {
      if (el.type === 'item') {
        if (current === item.parent) {
          continue
        }

        return el.props
      }

      const first = _findFirst(el.nav)

      if (first) {
        return first
      }
    }

    const parentNav = current.getParentNavigator()

    if (!parentNav) {
      return null
    }

    selfIndex = parentNav.getElements().findIndex((el) => el.type === 'row' && el.nav === current)

    if (selfIndex === -1) {
      throw new Error("Internal error: row not found in its parent's elements")
    }

    current = parentNav
  }
}

document.body.addEventListener('keydown', (e) => {
  if (e.shiftKey || e.altKey || e.ctrlKey) {
    return
  }

  navState.update((state): NavigationState | null => {
    if (!state) {
      return state
    }

    const initial = state.focusOn
    const current = initial
    let next: NavigableItemProps | null

    if (!current) {
      next = _findFirst(state.pageNav.subNavigator)

      if (!next) {
        console.warn('No element to focus in this view')
        return state
      }
    } else {
      switch (e.key) {
        case 'ArrowUp':
          next = _findVertical(current, 'up')
          break

        case 'ArrowLeft':
          next = _findHorizontal(current, 'left')
          break

        case 'ArrowRight':
          next = _findHorizontal(current, 'right')
          break

        case 'ArrowDown':
          next = _findVertical(current, 'down')
          break

        case 'Enter':
          current.onPress()
          return state

        // TODO: Long Press

        case 'Backspace':
        case 'Escape':
          // TODO: 'onBack' on all navigators?
          if (current.onBack) {
            current.onBack()
          } else {
            state.pageNav.onBack()
          }
          return state

        default:
          return state
      }
    }

    if (!next) {
      return state
    }

    current?.onFocusChange(false)
    next.onFocusChange(true)

    return {
      pageNav: state.pageNav,
      focusOn: next,
    }
  })
})
