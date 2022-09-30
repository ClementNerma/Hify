import { getContext, setContext } from 'svelte'
import { get, writable } from 'svelte/store'
import { logFatal, logWarn } from '../stores/debugger'
import { handleInput, KeyPressHandling, registerLongPressableKeys } from './input-manager'

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
  LongBack,
}

export enum NavigationComingFrom {
  Below,
  Left,
  Right,
  Above,
}

export type OnFocusChangeCallback = (isFocused: boolean) => void

export type NavigableCommonProps = {
  position: number | null
  hasFocusPriority: boolean | null
  onFocusChangeCallback?: OnFocusChangeCallback | null
}

export type Props<N extends NavigableCommon> = N['props']

export type NoProp = Record<string, never>

export abstract class NavigableCommon<P = {}> {
  readonly parent: NavigableContainer<unknown>
  readonly identity: symbol
  readonly page: NavigablePage

  protected focused = writable(false)

  constructor(parent: NavigableContainer<unknown> | symbol, protected _props: NavigableCommonProps & P) {
    if (!(parent instanceof NavigableContainer)) {
      if (parent !== PAGE_CTR_TOKEN) {
        throw new Error('Invalid page construction token provided!')
      }

      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      this.parent = undefined as any
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      this.identity = undefined as any
      // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
      this.page = undefined as any

      return
    }

    this.parent = parent
    this.identity = parent.identity
    this.page = parent.page
  }

  get props(): Readonly<NavigableCommonProps & P> {
    return this._props
  }

  abstract navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem<unknown> | null
  abstract navigateToLastItem(): NavigableItem<unknown> | null

  abstract canHandleAction(key: NavigationAction): boolean
  abstract handleAction(key: NavigationAction): NavigableItem<unknown> | null

  abstract requestFocus(): boolean

  updateProps(props: NavigableCommonProps & P): void {
    this._props = props
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars, @typescript-eslint/no-empty-function
  interceptKeyPress(_key: string, _long: boolean): KeyPressHandling | void {}
}

export abstract class NavigableContainer<P = {}> extends NavigableCommon<P> {
  abstract get ordered(): boolean

  abstract append(navigable: Navigable): void
  abstract hasChild(child: Navigable): boolean
  abstract remove(child: Navigable): void
  abstract navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem<unknown> | null

  canHandleAction(_: NavigationAction): boolean {
    return false
  }

  handleAction(_: NavigationAction): NavigableItem<unknown> | null {
    throw new Error('This navigable container does not support actions')
  }
}

export abstract class NavigableArrayContainer<P = {}> extends NavigableContainer<P> {
  protected _unorderedItems: Navigable[] = []

  get ordered(): boolean {
    return this._unorderedItems.length > 0 ? this._unorderedItems[0].props.position !== null : false
  }

  protected get items(): Navigable[] {
    return this.ordered
      ? [...this._unorderedItems].sort((a, b) => {
          if (a.props.position === null || b.props.position === null) {
            return logFatal('Internal error: position not definied in ordered items array')
          }

          return a.props.position - b.props.position
        })
      : this._unorderedItems
  }

  protected getFocusPriority(): Navigable | null {
    return this._unorderedItems.find((item) => item.props.hasFocusPriority === true) ?? null
  }

  requestFocus(): boolean {
    return this.items[0]?.requestFocus() ?? false
  }

  append(navigable: Navigable): void {
    if (this._unorderedItems.length > 0) {
      if (this.ordered && navigable.props.position === null) {
        throw new Error('Cannot append a non-positioned item to an ordered container')
      } else if (!this.ordered && navigable.props.position !== null) {
        throw new Error('Cannot append a positioned item to an unordered container')
      }
    }

    this._unorderedItems.push(navigable)
  }

  remove(child: Navigable): void {
    const indexOf = this._unorderedItems.indexOf(child)

    if (indexOf === -1) {
      throw new Error('Cannot remove unknown child')
    }

    this._unorderedItems.splice(indexOf, 1)
  }

  hasChild(child: Navigable): boolean {
    return this._unorderedItems.indexOf(child) !== -1
  }

  navigateToLastItem(): NavigableItem<unknown> | null {
    for (let c = this._unorderedItems.length - 1; c >= 0; c--) {
      const target = this._unorderedItems[c].navigateToLastItem()

      if (target) {
        return target
      }
    }

    return null
  }
}

export abstract class NavigableItem<P = {}> extends NavigableCommon<P> {
  abstract canHandleDirection(direction: NavigationDirection): boolean
  abstract handleDirection(direction: NavigationDirection): NavigableItem<unknown> | null

  abstract underlyingElement(): HTMLNavigableItemWrapperElement

  abstract onFocus(): void
  abstract onUnfocus(): void

  navigateToFirstItemDown(_: NavigationComingFrom): NavigableItem<unknown> {
    return this
  }

  navigateToLastItem(): NavigableItem<unknown> | null {
    return this
  }

  scrollTo(): void {
    const el = this.underlyingElement()

    if (el.constructor.name !== HTMLNavigableItemWrapperElement.name) {
      throw new Error("Item's underlying element is not an " + HTMLNavigableItemWrapperElement.name)
    }

    if (!el.children.length || !(el.children[0] instanceof HTMLElement)) {
      return logWarn('Failed to scroll to element has it does not have a valid child element')
    }

    if (el.children.length > 0) {
      el.children[0].scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'nearest' })
    } else {
      logWarn('Navigable item has no children ; cannot scroll into view')
    }
  }
}

class NavigablePage extends NavigableContainer {
  override readonly identity: symbol
  override readonly page: NavigablePage
  override readonly parent: NavigableContainer

  readonly priorityFocusables: NavigableItem<unknown>[] = []
  readonly ordered = false

  private onlyChild: Navigable | null = null

  constructor(
    private readonly onRequestFocus: (item: NavigableItem<unknown>) => void,
    private readonly getFocusedItem: () => NavigableItem<unknown> | null,
  ) {
    super(PAGE_CTR_TOKEN, { position: null, hasFocusPriority: null })

    this.identity = Symbol()
    this.page = this
    this.parent = this
  }

  override canHandleAction(_: NavigationAction): boolean {
    return false
  }

  override handleAction(_: NavigationAction): NavigableItem<unknown> | null {
    throw new Error('Tried to make the navigable page component handle an action')
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

  navigate(_: Navigable, __: NavigationDirection): NavigableItem<unknown> | null {
    return null
  }

  navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem<unknown> | null {
    return this.onlyChild ? this.onlyChild.navigateToFirstItemDown(from) : null
  }

  navigateToLastItem(): NavigableItem<unknown> | null {
    return this.onlyChild ? this.onlyChild.navigateToLastItem() : null
  }

  requestFocus(): boolean {
    return this.onlyChild?.requestFocus() ?? false
  }

  asContainer(): NavigableContainer {
    return this
  }

  requestPageFocus(item: NavigableItem<unknown>): void {
    if (item.parent.page !== this) {
      throw new Error("Cannot request focus for an element that isn't part of the same page")
    }

    this.onRequestFocus(item)
  }

  focusedItem(): NavigableItem<unknown> | null {
    const item = this.getFocusedItem()

    if (!item) {
      return null
    }

    if (item.parent.page !== this) {
      throw new Error("Cannot return item focused when it isn't part of the same page")
    }

    return item
  }
}

export function getParentNavigable(item?: true): NavigableContainer<unknown> {
  if (item) {
    if (Boolean(getContext(NAVIGABLE_ITEM_DETECTION_CTX))) {
      throw new Error('Cannot use a navigable inside an item')
    }

    setContext(NAVIGABLE_ITEM_DETECTION_CTX, true)
  }

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
  const page = new NavigablePage(_requestFocus, () => get(navState)?.focused ?? null)

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

export function handleKeyboardEvent(key: string, long: boolean): void {
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

  for (const item of _getItemChain(current)) {
    if (item.interceptKeyPress(key, long) === KeyPressHandling.Intercepted) {
      return
    }
  }

  let next: NavigableItem<unknown> | null

  switch (key) {
    case 'ArrowUp':
    case 'ArrowLeft':
    case 'ArrowRight':
    case 'ArrowDown':
      if (currentJustFocused) {
        next = current
        break
      }

      const directions: { [key in typeof key]: NavigationDirection } = {
        ArrowUp: NavigationDirection.Up,
        ArrowLeft: NavigationDirection.Left,
        ArrowRight: NavigationDirection.Right,
        ArrowDown: NavigationDirection.Down,
      }

      const direction = directions[key]

      next = current.canHandleDirection(direction)
        ? current.handleDirection(direction)
        : current.parent.navigate(current, direction)

      break

    case 'Enter':
    case 'Escape':
      const events: { [key in typeof key]: NavigationAction } = {
        Enter: long ? NavigationAction.LongPress : NavigationAction.Press,
        Escape: long ? NavigationAction.LongBack : NavigationAction.Back,
      }

      const event = events[key]

      if (currentJustFocused && event !== NavigationAction.Back) {
        next = current
        break
      }

      next = null

      for (const nav of _getItemChain(current)) {
        if (!nav.canHandleAction(event)) {
          continue
        }

        const newFocused = nav.handleAction(event)

        if (newFocused) {
          next = newFocused
        }

        break
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
    navState.set(_generateUpdatedNavState(current, next, state.page))
  }
}

function _getItemChain(item: NavigableItem<unknown>): Navigable[] {
  const out: Navigable[] = [item]

  let current: NavigableContainer = item.parent

  while (!(current instanceof NavigablePage)) {
    out.push(current)
    current = current.parent
  }

  return out
}

function _checkItemValidity(item: NavigableItem<unknown>, page: NavigablePage): boolean {
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

function _requestFocus(item: NavigableItem<unknown>): void {
  navState.update((state) =>
    state && _checkItemValidity(item, state.page) ? _generateUpdatedNavState(state.focused, item, state.page) : state,
  )
}

function _generateUpdatedNavState(
  oldFocused: NavigableItem<unknown> | null,
  newFocused: NavigableItem<unknown>,
  page: NavigablePage,
): NavState {
  if (oldFocused) {
    oldFocused.onUnfocus()
    _propagateFocusChangeEvent(oldFocused, false)
  }

  newFocused.scrollTo()
  newFocused.onFocus()
  _propagateFocusChangeEvent(newFocused, true)

  return { page, focused: newFocused }
}

function _propagateFocusChangeEvent(item: NavigableItem<unknown>, focused: boolean): void {
  for (const subItem of _getItemChain(item)) {
    subItem.props.onFocusChangeCallback?.(focused)
  }
}

export type RequestFocus = () => boolean

export type Navigable = NavigableContainer | NavigableItem<unknown>

const NAVIGATION_CTX = Symbol()
const NAVIGABLE_ITEM_DETECTION_CTX = Symbol()
const PAGE_CTR_TOKEN = Symbol()

type NavState = {
  page: NavigablePage
  focused: NavigableItem<unknown> | null
}

const navState = writable<NavState | null>(null)

export class HTMLNavigableItemWrapperElement extends HTMLElement {}

const itemWrapperInPlace = window.customElements.get('navigable-item-wrapper')

if (!itemWrapperInPlace) {
  window.customElements.define('navigable-item-wrapper', HTMLNavigableItemWrapperElement)
} else if (itemWrapperInPlace.name !== HTMLNavigableItemWrapperElement.name) {
  throw new Error('An invalid item wrapper element is already in place')
}

// Support long-press for "Enter" key
registerLongPressableKeys('Enter')

handleInput(handleKeyboardEvent)
