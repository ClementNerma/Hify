// TODO: add a "removed" marker alongside "state", set when item is unregistered + propagated to all its children
// TODO: throw if root item is unregistered (in debug mode as it requires checking the DOM)

import { showFailure } from '#/global/notifications.ts'
import { randomId } from '#/utils/common.ts'

export type Navigable<NavType extends string, Props extends UntypedNavigableProps, State> = {
  readonly id: string // TODO: make this something that can't be built from the outside?
  readonly navigableType: NavType
  readonly props: Props
  readonly state: State
  readonly helpers: NavigationHelpers
}

export type UntypedNavigable = Navigable<string, UntypedNavigableProps, UntypedNavigableState>
export type UntypedNavigableProps = Record<string, unknown>
export type UntypedNavigableState = Record<string, unknown>

// TODO: make 'on{Un}FocusedChild' only present if 'isContainer: true'
export type NavigableHandler<
  NavType extends string,
  Props extends UntypedNavigableProps,
  State extends UntypedNavigableState,
> = {
  navType: NavType
  isContainer: boolean

  // validateProps(untypedProps: UntypedNavigableProps): Props | Error

  createState(): State
} & NavigableHandlerMethods<NavType, Props, State>

export type NavigableHandlerMethods<
  NavType extends string,
  Props extends UntypedNavigableProps,
  State,
> = {
  enterFrom(
    nav: Navigable<NavType, Props, State>,
    fromDir: NavigationComingFromDirection,
  ): NavigationResult

  navigate(
    nav: Navigable<NavType, Props, State>,
    focusedChild: UntypedNavigable,
    dir: NavigationDirection,
  ): NavigationResult

  handleKeyPress?(
    nav: Navigable<NavType, Props, State>,
    key: NavigationKeyName,
  ): NavigationResult | null

  onFocused?(nav: Navigable<NavType, Props, State>): void

  onUnfocused?(nav: Navigable<NavType, Props, State>): void

  // For containers only
  onFocusedChild?(
    nav: Navigable<NavType, Props, State>,
    newlyFocusedChild: UntypedNavigable,
    parentNewlyFocused: boolean, // Is the current container newly focused
  ): void

  // For containers only
  onUnfocusedChild?(
    nav: Navigable<NavType, Props, State>,
    unfocusedChild: UntypedNavigable,
    willUnfocusParent: boolean, // Is the current container going to be unfocused
  ): void
}

export type NavigationHelpers = {
  focus(dir: NavigationDirection): void
  focusAnother(id: string, dir: NavigationDirection): void
  findChildren(): UntypedNavigable[]
  // findDescendantsById(id: string): UntypedNavigable | null
  findParent(): UntypedNavigable | null
  findAncestors(): UntypedNavigable[]
  findSiblings(): UntypedNavigable[]
}

export type NavigationResult =
  | { type: 'cancel' }
  | { type: 'focusThis' }
  | {
      type: 'focusChild'
      navId: string
      fromDir: NavigationComingFromDirection | null
    }
  | { type: 'propagate' }

export type NavigationDirection = 'UP' | 'DOWN' | 'LEFT' | 'RIGHT'

export type NavigationComingFromDirection = 'ABOVE' | 'BELOW' | 'LEFT' | 'RIGHT' | 'NOWHERE'

export type NavigationKeyName =
  | 'UP'
  | 'DOWN'
  | 'LEFT'
  | 'RIGHT'
  | 'BACK'
  | 'SHORT_PRESS'
  | 'LONG_PRESS'

// export function oppositeNavigableDirection(dir: NavigationDirection): NavigationDirection {
//   const tr: Record<NavigationDirection, NavigationDirection> = {
//     UP: 'DOWN',
//     LEFT: 'RIGHT',
//     RIGHT: 'LEFT',
//     DOWN: 'UP',
//   }

//   return tr[dir]
// }

export type UntypedNavigableHandler = NavigableHandler<
  string,
  UntypedNavigableProps,
  UntypedNavigableState
>

export type NavigableOf<T extends UntypedNavigableHandler> =
  T extends NavigableHandler<infer N, infer P, infer S> ? Navigable<N, P, S> : never

export type UntypedNavigablesSet = Record<string, UntypedNavigableProps>

//
// => Utility types for registries
//

export type RegistryItems<R extends UntypedNavigablesSet> = {
  [NavType in keyof R & string]: Navigable<NavType, R[NavType], UntypedNavigableState>
}

export type RegistryItemNavType<R extends UntypedNavigablesSet> = keyof RegistryItems<R>

export type RegistryItem<
  R extends UntypedNavigablesSet,
  NavType extends keyof R & string,
> = RegistryItems<R>[NavType] & { extendedProps: ExtendedNavProps }

export type RegistryItemProps<
  R extends UntypedNavigablesSet,
  NavType extends keyof R & string,
> = RegistryItemBaseProps<R, NavType> & ExtendedNavProps

export type RegistryItemBaseProps<
  R extends UntypedNavigablesSet,
  NavType extends keyof R & string,
> = R[NavType]

export type AnyRegistryItem<R extends UntypedNavigablesSet> =
  RegistryItems<R>[keyof RegistryItems<R>] & { extendedProps: ExtendedNavProps }

export type RegistryHandlers<R extends UntypedNavigablesSet> = {
  [NavType in keyof R & string]: NavigableHandler<NavType, R[NavType], UntypedNavigableState>
}

export type RegistryHandler<
  R extends UntypedNavigablesSet,
  NavType extends keyof R & string,
> = RegistryHandlers<R>[NavType]

export type RegistryHandlerMethods<
  R extends UntypedNavigablesSet,
  NavType extends keyof R & string,
> = NavigableHandlerMethods<NavType, R[NavType], UntypedNavigableState>

export type AnyRegistryHandler<R extends UntypedNavigablesSet> =
  RegistryHandlers<R>[keyof RegistryHandlers<R>]

export type ExtendedNavProps = {
  onFocused?: (() => void) | undefined
  onUnfocused?: (() => void) | undefined

  onLeftKey?: (() => void | { type: 'propagate' }) | undefined
  onRightKey?: (() => void | { type: 'propagate' }) | undefined
  onUpKey?: (() => void | { type: 'propagate' }) | undefined
  onDownKey?: (() => void | { type: 'propagate' }) | undefined
  onPress?: (() => void | { type: 'propagate' }) | undefined
  onLongPress?: (() => void | { type: 'propagate' }) | undefined
  onBackKey?: (() => void | { type: 'propagate' }) | undefined
  onLongBackKey?: (() => void | { type: 'propagate' }) | undefined
  onBeginningKey?: (() => void | { type: 'propagate' }) | undefined
  onEndKey?: (() => void | { type: 'propagate' }) | undefined
}

// oxlint-disable-next-line typescript/no-explicit-any
type ReturnTypeOfUndefinableFn<F extends ((...args: any[]) => any) | undefined> =
  F extends undefined
    ? ReturnType<Exclude<F, undefined>> | undefined
    : ReturnType<Exclude<F, undefined>>

// TODO: create a method to be triggered when props change for an item

export class NavigationManager<R extends UntypedNavigablesSet> {
  private readonly domContainer: HTMLElement
  private readonly navigableHandlers: RegistryHandlers<R>
  private readonly navById = new Map<
    string,
    AnyRegistryItem<R> & { extendedProps: ExtendedNavProps }
  >()

  constructor(
    domContainer: typeof this.domContainer,
    navigableHandlers: typeof this.navigableHandlers,
  ) {
    this.domContainer = domContainer
    this.navigableHandlers = navigableHandlers
  }

  createNav<NavType extends RegistryItemNavType<R>>(
    navigableType: NavType,
    mergedProps: RegistryItemProps<R, NavType>,
    specificId?: string,
  ): string {
    if (!Object.hasOwn(this.navigableHandlers, navigableType)) {
      throw new Error(`Cannot create unknown navigable type "${navigableType}"`)
    }

    if (specificId !== undefined && this.navById.has(specificId)) {
      return specificId
      // throw new Error(`Cannot create navigable with duplicate ID "${specificId}"`)
    }

    const id = specificId ?? `${navigableType}_${randomId()}`

    const handler = this.getUntypedHandler(navigableType)

    const helpers: NavigationHelpers = {
      focus: (dir) => this.focusById(id, dir),
      focusAnother: (anotherId, dir) => this.focusById(anotherId, dir),
      findChildren: () => this.findChildrenOf(id),
      findParent: () => this.findParentOf(id),
      findAncestors: () => this.findAncestorsOf(id),
      findSiblings: () => this.findSiblingsOf(id),
    }

    const { props, extendedProps } = _categorizeProps(mergedProps)

    const nav: RegistryItem<R, NavType> = {
      id,
      navigableType,
      state: handler.createState(),
      props,
      helpers,
      extendedProps,
    }

    this.navById.set(id, nav)

    return nav.id
  }

  updateNavProps<NavType extends RegistryItemNavType<R>>(
    navId: string,
    navType: NavType,
    newMergedProps: RegistryItemProps<R, NavType>,
  ): void {
    const nav = this.get(navId)

    if (nav.navigableType !== navType) {
      throw new Error(
        `Cannot update navigable props: navigable type mismatch (navigable has type "${nav.navigableType}", was provided "${navType}")`,
      )
    }

    const { props, extendedProps } = _categorizeProps(newMergedProps)

    // TODO: fix the need to remove the readonly marker
    type Writeable<T> = { -readonly [P in keyof T]: T[P] }
    // oxlint-disable-next-line typescript/no-unsafe-type-assertion
    ;(nav.props as Writeable<typeof nav.props>) = props
    ;(nav.extendedProps as Writeable<typeof nav.extendedProps>) = extendedProps
  }

  unregisterNav(navId: string): void {
    if (!this.navById.has(navId)) {
      throw new Error(`Cannot unregister unknown navigable with ID "${navId}"`)
    }

    // TODO: if this item is the focused one, unfocus it first
    //
    // But due to the fact unmount hooks run from top (parents) to bottom (children),
    //  the parents will be unregistered before the focused descendant

    this.navById.delete(navId)
  }

  focusedId(): string | null {
    const focused = this.getFocused()
    return focused ? focused.id : null
  }

  focusById(navId: string, dir: NavigationDirection | null): void {
    const focused = this.getFocused()

    if (focused?.id === navId) {
      return
    }

    const target = this.get(navId)

    if (this.getUntypedHandler(target.navigableType).isContainer) {
      this.enterContainer(navId, dir)
      return
    }

    if (focused) {
      const { commonParents, fromOnlyParents, toOnlyParents } = this.categorizeParentsDifference(
        focused.id,
        target.id,
      )

      _assert(commonParents.length > 0, 'There should be at least one common parent (the root)')

      //
      // => Unfocusing
      //

      focused.extendedProps.onUnfocused?.()
      this.callHandlerMethod(focused, 'onUnfocused')
      this.findDomById(focused.id).removeAttribute(NAVIGABLE_DATA_FOCUSED_ATTR)

      if (fromOnlyParents.length === 0) {
        const parent = _unwrapUndefinable(commonParents[0]) // last common
        this.callHandlerMethod(parent, 'onUnfocusedChild', focused, false)
      }

      for (const [child, parent] of _oneShiftedList(fromOnlyParents, focused)) {
        this.callHandlerMethod(parent, 'onUnfocusedChild', child, true)
        this.callHandlerMethod(parent, 'onUnfocused')
      }

      //
      // => Focusing
      //

      for (const [child, parent] of _oneShiftedList(toOnlyParents, target)) {
        this.callHandlerMethod(parent, 'onFocusedChild', child, true)
        this.callHandlerMethod(parent, 'onFocused')
      }

      if (toOnlyParents.length === 0) {
        const parent = _unwrapUndefinable(commonParents[0]) // last common
        this.callHandlerMethod(parent, 'onFocusedChild', target, false)
      }
    } else {
      for (const [child, parent] of _oneShiftedList(this.findAncestorsOf(navId), target)) {
        this.callHandlerMethod(parent, 'onFocusedChild', child, true)
        this.callHandlerMethod(parent, 'onFocused')
      }
    }

    target.extendedProps.onFocused?.()
    this.callHandlerMethod(target, 'onFocused')

    const targetDOM = this.findDomById(target.id)
    targetDOM.setAttribute(NAVIGABLE_DATA_FOCUSED_ATTR, 'true')

    targetDOM.scrollIntoView({
      behavior: 'instant',
      block: 'nearest',
      inline: 'nearest',
    })
  }

  focusChildOf(navId: string, childIndex: number, dir: NavigationDirection | null): void {
    const nav = this.get(navId)

    const children = this.findChildrenOf(nav.id)

    if (childIndex < 0 || childIndex >= children.length) {
      throw new Error(`Cannot focus child at invalid index ${childIndex} for navigable "${navId}"`)
    }

    this.focusById(children[childIndex].id, dir)
  }

  unfocus(): void {
    const focused = this.getFocused()

    if (!focused) {
      return
    }

    const ancestors = this.findAncestorsOf(focused.id)

    for (const [child, parent] of _oneShiftedList(ancestors, focused)) {
      this.callHandlerMethod(parent, 'onUnfocusedChild', child, true)
      this.callHandlerMethod(parent, 'onUnfocused')
    }

    this.callHandlerMethod(focused, 'onUnfocused')
    this.findDomById(focused.id).removeAttribute(NAVIGABLE_DATA_FOCUSED_ATTR)
  }

  private getFocused(): AnyRegistryItem<R> | null {
    const focused = this.domContainer.querySelector(`[${NAVIGABLE_DATA_FOCUSED_ATTR}="true"]`)

    return focused ? this.getNavigableFromDOM(focused) : null
  }

  dispatchKeyPress(key: NavigationKeyName): void {
    const focused = this.getFocused()

    if (!focused) {
      this.focusFirstItem()
      return
    }

    const navResult = this.callKeyPressHandler(focused, key)

    if (navResult) {
      const keysAsNavDirection = {
        UP: 'UP',
        DOWN: 'DOWN',
        LEFT: 'LEFT',
        RIGHT: 'RIGHT',
        BACK: null,
        SHORT_PRESS: null,
        LONG_PRESS: null,
      } as const

      this.handleNavigationResult(focused, navResult, keysAsNavDirection[key])
      return
    }

    switch (key) {
      case 'UP':
      case 'LEFT':
      case 'RIGHT':
      case 'DOWN': {
        const parent = this.findParentOf(focused.id)

        if (!parent) {
          return
        }

        const parentNavResult = this.callHandlerMethod(parent, 'navigate', focused, key)

        this.handleNavigationResult(parent, parentNavResult, key)

        break
      }

      case 'BACK':
      case 'SHORT_PRESS':
      case 'LONG_PRESS': {
        let currNav = focused

        while (true) {
          const parentNav = this.findParentOf(currNav.id)

          // Stop after root
          if (!parentNav) {
            break
          }

          const navResult = this.callKeyPressHandler(parentNav, key)

          if (navResult) {
            this.handleNavigationResult(parentNav, navResult, null)
            return
          }

          currNav = parentNav
        }

        break
      }

      default: {
        _typecheckUnreachable(key)
      }
    }
  }

  private handleNavigationResult(
    currItem: AnyRegistryItem<R>,
    action: NavigationResult,
    dir: NavigationDirection | null,
  ): void {
    switch (action.type) {
      case 'cancel': {
        return
      }

      case 'focusThis': {
        this.focusById(currItem.id, dir)
        break
      }

      case 'focusChild': {
        this.focusById(action.navId, dir)
        return
      }

      case 'propagate': {
        const siblings = this.findSiblingsOf(currItem.id)

        const siblingIndex = siblings.findIndex((c) => c.id === currItem.id)

        if (siblingIndex === -1) {
          console.error(currItem.id, siblings)
          throw new Error('Unexpected: navigable item not found in its siblings list')
        }

        const parent = this.findParentOf(currItem.id)

        // If there is no parent (root item), don't do anything
        if (!parent) {
          break
        }

        if (dir === null) {
          if (siblingIndex < siblings.length - 1) {
            this.focusById(siblings[siblingIndex + 1].id, null)
          } else {
            this.handleNavigationResult(parent, { type: 'propagate' }, null)
          }

          break
        }

        const result = this.callHandlerMethod(parent, 'navigate', currItem, dir)

        this.handleNavigationResult(parent, result, dir)
        break
      }

      default: {
        _typecheckUnreachable(action)
      }
    }
  }

  callKeyPressHandler<NavType extends RegistryItemNavType<R>>(
    nav: RegistryItem<R, NavType>,
    key: NavigationKeyName,
  ): NavigationResult | null {
    const { extendedProps, ...baseNav } = nav

    const extendedMethodNames: Record<NavigationKeyName, keyof ExtendedNavProps> = {
      UP: 'onUpKey',
      DOWN: 'onDownKey',
      LEFT: 'onLeftKey',
      RIGHT: 'onRightKey',
      SHORT_PRESS: 'onPress',
      LONG_PRESS: 'onLongPress',
      BACK: 'onBackKey',
    }

    const extendedMethod = extendedProps[extendedMethodNames[key]]

    if (extendedMethod) {
      const result = extendedMethod()

      if (result === undefined) {
        return { type: 'cancel' }
      }
    }

    const handler = this.getUntypedHandler(nav.navigableType)

    if (!handler.handleKeyPress) {
      return null
    }

    return handler.handleKeyPress(
      // @ts-expect-error TypeScript cannot infer that nav is of the correct type
      baseNav,
      key,
    )
  }

  callHandlerMethod<
    NavType extends RegistryItemNavType<R>,
    MethodName extends Exclude<keyof RegistryHandlerMethods<R, NavType>, 'handleKeyPress'>,
  >(
    nav: RegistryItem<R, NavType>,
    methodName: MethodName,
    ...args: RemoveFirstFromTuple<
      Parameters<Exclude<RegistryHandlerMethods<R, NavType>[MethodName], undefined>>
    >
  ): ReturnTypeOfUndefinableFn<RegistryHandlerMethods<R, NavType>[MethodName]> {
    const handler = this.getUntypedHandler(nav.navigableType)

    const { extendedProps, ...baseNav } = nav

    switch (methodName) {
      case 'onFocused': {
        extendedProps.onFocused?.()
        break
      }

      case 'onUnfocused': {
        extendedProps.onUnfocused?.()
        break
      }

      case 'enterFrom':
      case 'navigate':
      case 'onFocusedChild':
      case 'onUnfocusedChild': {
        break
      }
    }

    // @ts-expect-error TS cannot infer that methodName is a valid key of handler
    return handler[methodName]?.(baseNav, ...args)
  }

  focusFirstItem(): void {
    const firstItem = this.domContainer.querySelector(`[${NAVIGABLE_DATA_ID_ATTR}]`)

    if (!firstItem) {
      showFailure('Cannot focus first item: no navigable items found')
      return
    }

    const nav = this.getNavigableFromDOM(firstItem)

    return this.focusById(nav.id, null)
  }

  private enterContainer(navId: string, dir: NavigationDirection | null): void {
    const nav = this.get(navId)

    const tr: Record<NavigationDirection, NavigationComingFromDirection> = {
      UP: 'BELOW',
      LEFT: 'RIGHT',
      RIGHT: 'LEFT',
      DOWN: 'ABOVE',
    }

    const from = dir === null ? 'NOWHERE' : tr[dir]

    const result = this.callHandlerMethod(nav, 'enterFrom', from)

    this.handleNavigationResult(nav, result, dir)
  }

  private get(navId: string): AnyRegistryItem<R> {
    const nav = this.navById.get(navId)

    if (!nav) {
      throw new Error(`Navigable with ID "${navId}" was found in DOM but is not registered`)
    }

    return nav
  }

  private getNavigableFromDOM(domEl: Element): AnyRegistryItem<R> {
    const navId = domEl.getAttribute(NAVIGABLE_DATA_ID_ATTR)

    if (navId === null) {
      throw new Error('DOM element is not associated with any navigable')
    }

    return this.get(navId)
  }

  findDomById(navId: string): Element {
    const nav = this.navById.get(navId)

    if (!nav) {
      throw new Error(`Navigable with ID "${navId}" was not found`)
    }

    const domEl = this.domContainer.querySelector(`[${NAVIGABLE_DATA_ID_ATTR}="${nav.id}"]`)

    if (!domEl) {
      throw new Error(`DOM element for navigable with ID "${nav.id}" was not found`)
    }

    return domEl
  }

  private findParentOf(navId: string): AnyRegistryItem<R> | null {
    const dom = this.findDomById(navId)
    const parent = dom.closest(
      `[${NAVIGABLE_DATA_ID_ATTR}]:not([${NAVIGABLE_DATA_ID_ATTR}="${navId}"])`,
    )

    return parent ? this.getNavigableFromDOM(parent) : null
  }

  private findAncestorsOf(navId: string): AnyRegistryItem<R>[] {
    const parents: AnyRegistryItem<R>[] = []

    let currEl = this.findDomById(navId)
    let currNavId = navId

    while (true) {
      const parentEl = currEl.closest(
        `[${NAVIGABLE_DATA_ID_ATTR}]:not([${NAVIGABLE_DATA_ID_ATTR}="${currNavId}"])`,
      )

      if (!parentEl) {
        break
      }

      const parentNav = this.getNavigableFromDOM(parentEl)

      parents.push(parentNav)

      currEl = parentEl
      currNavId = parentNav.id
    }

    return parents
  }

  private findChildrenOf(navId: string): AnyRegistryItem<R>[] {
    const navDom = this.findDomById(navId)

    const children = [
      ...navDom.querySelectorAll(
        // Find all navigable elements that are not descendants of other navigable elements inside the parent one
        `[${NAVIGABLE_DATA_ID_ATTR}]:not([${NAVIGABLE_DATA_ID_ATTR}="${navId}"] [${NAVIGABLE_DATA_ID_ATTR}] *)`,
      ),
    ]

    return children.map((dom) => this.getNavigableFromDOM(dom))
  }

  private findSiblingsOf(navId: string): AnyRegistryItem<R>[] {
    const parent = this.findParentOf(navId)

    if (parent) {
      return this.findChildrenOf(parent.id)
    }

    // The root item should not have siblings
    return [this.get(navId)]
  }

  private categorizeParentsDifference(
    fromNavId: string,
    toNavId: string,
  ): {
    commonParents: AnyRegistryItem<R>[]
    fromOnlyParents: AnyRegistryItem<R>[]
    toOnlyParents: AnyRegistryItem<R>[]
  } {
    const fromParents = this.findAncestorsOf(fromNavId)
    const toParents = this.findAncestorsOf(toNavId)

    const fromParentsId = new Set(fromParents.map((nav) => nav.id))
    const toParentsId = new Set(toParents.map((nav) => nav.id))

    return {
      commonParents: fromParents.filter((nav) => toParentsId.has(nav.id)),
      fromOnlyParents: fromParents.filter((nav) => !toParentsId.has(nav.id)),
      toOnlyParents: toParents.filter((nav) => !fromParentsId.has(nav.id)),
    }
  }

  private getUntypedHandler(navType: string): AnyRegistryHandler<R> {
    return this.getTypedHandler(navType)
  }

  private getTypedHandler<NavType extends RegistryItemNavType<R>>(
    navType: NavType,
  ): RegistryHandler<R, NavType> {
    if (!Object.hasOwn(this.navigableHandlers, navType)) {
      throw new Error(`Cannot get navigable handler for unknown navigable type "${navType}"`)
    }

    return this.navigableHandlers[navType]
  }
}

export const NAVIGABLE_DATA_ID_ATTR = 'data-navigable-id'
export const NAVIGABLE_DATA_TYPE_ATTR = 'data-navigable-type'
export const NAVIGABLE_DATA_FOCUSED_ATTR = 'data-navigable-focused'

function _assert(condition: boolean, message: string): asserts condition {
  if (!condition) {
    throw new Error(`Assertion failed: ${message}`)
  }
}

function _unwrapUndefinable<T>(value: T | undefined): T {
  if (value === undefined) {
    throw new Error('Unexpected undefined value')
  }

  return value
}

function _typecheckUnreachable(_: never): never {
  console.error({ never: _ })
  throw new Error('Reached theorically unreachable statement')
}

function _oneShiftedList<T>(array: T[], withFirst: T): [T, T][] {
  return array.map((value, i) => [i === 0 ? withFirst : array[i - 1], value])
}

function _categorizeProps<R extends UntypedNavigablesSet, N extends keyof R & string>(
  props: R[N] & ExtendedNavProps,
): { props: R[N]; extendedProps: ExtendedNavProps } {
  const {
    onFocused,
    onUnfocused,
    onUpKey,
    onDownKey,
    onLeftKey,
    onRightKey,
    onPress,
    onLongPress,
    onBackKey,
    onLongBackKey,
    onBeginningKey,
    onEndKey,
    ...baseProps
  } = props

  return {
    // oxlint-disable-next-line typescript/no-unsafe-type-assertion
    props: baseProps as R[N],
    extendedProps: {
      onFocused,
      onUnfocused,
      onUpKey,
      onDownKey,
      onLeftKey,
      onRightKey,
      onPress,
      onLongPress,
      onBackKey,
      onLongBackKey,
      onBeginningKey,
      onEndKey,
    },
  }
}

// oxlint-disable-next-line typescript/no-explicit-any
type RemoveFirstFromTuple<T extends any[]> = T extends [any, ...infer R] ? R : never
