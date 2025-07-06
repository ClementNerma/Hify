export type SetupNavigableOptions = {
	log: (level: LogLevel, message: string, error?: unknown) => void
	inputHandler?: (keyDownListener: (e: KeyboardEvent) => void, keyUpListener: (e: KeyboardEvent) => void) => void
	keyLongPressThresholdMs?: number
}

export enum LogLevel {
	Debug = 'Debug',
	Info = 'Info',
	Warn = 'Warn',
	Error = 'Error',
	Fatal = 'Fatal',
}

let setupOptions: SetupNavigableOptions | null = null

function assertNever(value: never): never {
	logFatal(`Reached theorically unreachable statement with value "${value}"`)
}

export function log(level: LogLevel, message: string, error?: unknown) {
	getSetupOptions()?.log?.(level, message, error)
}

export function logFatal(message: string, error?: unknown): never {
	console.error(error)
	throw new Error(`[Navigable] ${message}`)
}

export function setupNavigable(options: SetupNavigableOptions) {
	setupOptions = options

	window.customElements.define('navigable-element-wrapper', HTMLNavigableElementWrapper)

	watchLongPressForKeys(['Enter'])

	if (options.inputHandler) {
		options.inputHandler(handleKeyDownEvent, handleKeyUpEvent)
	} else {
		document.body.addEventListener('keydown', handleKeyDownEvent)
		document.body.addEventListener('keyup', handleKeyUpEvent)
	}
}

function getSetupOptions(): SetupNavigableOptions {
	if (!setupOptions) {
		throw new Error('Setup options for navigables are not set')
	}

	return setupOptions
}

const pendingKeyLongPresses = new Map<string, { at: number; timeout: number } | null>()
const watchingLongPressForKeys = new Set<string>()
const triggeredKeyLongPress = new Set<string>()

const DEFAULT_KEY_LONG_PRESS_THRESHOLD_MS = 250

function handleKeyDownEvent(e: KeyboardEvent): void {
	const { key, ctrlKey, shiftKey, altKey } = e

	if (key === 'Enter' || translateNavigationKey(key)) {
		e.preventDefault()
	}

	const modifiers = { ctrlKey, shiftKey, altKey } satisfies Partial<KeyPress>

	if (watchingLongPressForKeys.has(key)) {
		if (!pendingKeyLongPresses.has(key) && !triggeredKeyLongPress.has(key)) {
			pendingKeyLongPresses.set(key, {
				at: performance.now(),
				timeout: window.setTimeout(() => {
					dispatchKeyInput({ key, longPress: true, ...modifiers })
					triggeredKeyLongPress.add(key)
					pendingKeyLongPresses.delete(key)
				}, setupOptions?.keyLongPressThresholdMs ?? DEFAULT_KEY_LONG_PRESS_THRESHOLD_MS),
			})
		}
	} else {
		dispatchKeyInput({ key, longPress: false, ...modifiers })
	}
}

function handleKeyUpEvent(e: KeyboardEvent): void {
	const { key, ctrlKey, shiftKey, altKey } = e

	if (!watchingLongPressForKeys.has(key)) {
		return
	}

	if (triggeredKeyLongPress.delete(key)) {
		return
	}

	const pending = pendingKeyLongPresses.get(key)

	if (pending === null || pending === undefined) {
		logFatal(`Internal error: timeout of keydown event for key "${key}" is not initialized`)
	}

	clearTimeout(pending.timeout)
	pendingKeyLongPresses.delete(key)

	const longPressThresholdMs = setupOptions?.keyLongPressThresholdMs ?? DEFAULT_KEY_LONG_PRESS_THRESHOLD_MS
	const longPress = performance.now() - pending.at > longPressThresholdMs

	dispatchKeyInput({ key, longPress, ctrlKey, shiftKey, altKey })
}

export function watchLongPressForKeys(keys: string[]): void {
	for (const key of keys) {
		watchingLongPressForKeys.add(key)
	}
}

export type KeyPress = {
	key: string
	longPress: boolean
	ctrlKey: boolean
	shiftKey: boolean
	altKey: boolean
}

export type InputHandler = (key: KeyPress) => InputHandlingResult | void

const inputHandlers: { handler: InputHandler; priority: number }[] = []

function dispatchKeyInput(key: KeyPress) {
	let propagateToElements = true

	// Run handlers by priority order (descending)
	for (const { handler } of inputHandlers.toSorted((a, b) => b.priority - a.priority)) {
		const result = handler(key)

		if (result === InputHandlingResult.Intercepted) {
			return
		}

		if (result === InputHandlingResult.PropagateToHandlersOnly) {
			propagateToElements = false
		}
	}

	if (propagateToElements) {
		handleKeyPress(key)
	}
}

export function handleInput(handler: InputHandler, priority: number): void {
	inputHandlers.push({ handler, priority })
}

export enum InputHandlingResult {
	Intercepted = 'INTERCEPTED',
	Propagate = 'PROPAGATE',
	PropagateToHandlersOnly = 'PROPAGATE_TO_HANDLERS_ONLY',
}

export enum NavigationDirection {
	Up = 'UP',
	Down = 'DOWN',
	Left = 'LEFT',
	Right = 'RIGHT',
	Back = 'BACK',
	Beginning = 'BEGINNING',
	End = 'END',
	DirectFocus = 'DIRECT_FOCUS',
}

export type NavigableCommonElementProps = {
	id: string
	disableScroll?: boolean
	// TODO: implement "trapped" here?
}

export type NavigableElement = NavigableCommonElementProps &
	(
		| { type: 'item'; hasFocusPriority?: boolean }
		| { type: 'list' }
		| { type: 'row' }
		| { type: 'grid'; columns: number }
		| { type: 'customContainer'; customId?: string }
	)

export type NavigableElementType = NavigableElement['type']

export type NavigableElementByType<ElementType extends NavigableElementType> = {
	[SubType in ElementType]: Extract<NavigableElement, { type: SubType }>
}[ElementType]

export type NavigableItemType = 'item'
export type NavigableContainerType = Exclude<NavigableElementType, 'item'>

export type NavigableContainer = Exclude<NavigableElement, { type: 'item' }>
export type NavigableItem = NavigableElementByType<'item'>

type ParamParser<T> = (value: string | undefined) => T | Error

const PARAM_PARSERS = {
	required: <T>(parser: (value: string) => T | Error): ParamParser<T> => {
		return (value) => (value === undefined ? new Error('no value provided for parameter') : parser(value))
	},

	optional: <T>(parser: (value: string) => T | Error): ParamParser<T | undefined> => {
		return (value) => (value === undefined ? undefined : parser(value))
	},

	string: (value: string) => value,

	positiveInt: (value: string) => {
		const parsed = Number.parseInt(value)

		return Number.isSafeInteger(parsed) && parsed > 0
			? parsed
			: new Error(`expected a positive integer, got: "${value}"`)
	},

	bool: (value: string) =>
		value === 'true' ? true : value === 'false' ? false : new Error(`expected a boolean, got: "${value}"`),
}

const COMMON_ELEMENTS_PROPS_PARSER = {
	disableScroll: PARAM_PARSERS.optional(PARAM_PARSERS.bool),
} satisfies {
	[ParamName in keyof Omit<NavigableCommonElementProps, 'id'>]: ParamParser<NavigableCommonElementProps[ParamName]>
}

const ELEMENTS_PARSER = {
	item: {
		hasFocusPriority: PARAM_PARSERS.optional(PARAM_PARSERS.bool),
	},
	list: {},
	row: {},
	grid: {
		columns: PARAM_PARSERS.required(PARAM_PARSERS.positiveInt),
	},
	customContainer: {
		customId: PARAM_PARSERS.optional(PARAM_PARSERS.string),
	},
} satisfies {
	[ElementType in NavigableElementType]: {
		[ParamName in Exclude<
			keyof NavigableElementByType<ElementType>,
			keyof NavigableCommonElementProps | 'type'
		>]: ParamParser<NavigableElementByType<ElementType>[ParamName]>
	}
}

export type NavigationResult = { type: 'focusItem'; item: NavigableItem } | { type: 'propagate' } | { type: 'trap' }
export type NavigationNativeFallbackResult = { type: 'native' }

export type NavigableItemInteractionHandlers = {
	focus(item: NavigableItem): void
	unfocus(item: NavigableItem): void
	press(item: NavigableItem): void
	longPress(item: NavigableItem): void
	interceptKeyPress(item: NavigableItem, key: KeyPress): NavigationResult
}

export type NavigableContainerInteractionHandlers<T extends NavigableContainerType> = {
	focus(navEl: NavigableElementByType<T>, focusedChild: NavigableElement): void
	unfocus(navEl: NavigableElementByType<T>, previouslyFocusedChild: NavigableElement): void
	navigate(
		navEl: NavigableElementByType<T>,
		currentChild: NavigableElement,
		direction: NavigationDirection,
	): NavigationResult
	enterFrom(navEl: NavigableElementByType<T>, from: NavigationDirection): NavigationResult
	interceptKeyPress(
		navEl: NavigableElementByType<T>,
		key: KeyPress,
		currentlyFocusedChild: NavigableElement,
	): NavigationResult
}

type _NavigableElementInteractionHandlers<T extends NavigableElementType> = T extends NavigableContainerType
	? NavigableContainerInteractionHandlers<T>
	: NavigableItemInteractionHandlers

export type NavigableElementInteractionHandlers<ElementType extends NavigableElementType> = {
	[SubType in ElementType]: _NavigableElementInteractionHandlers<SubType>
}[ElementType]

function _structElementEvtHandlers<ElementType extends NavigableElementType>(
	_: ElementType,
	generateHandlers: () => NavigableElementInteractionHandlers<ElementType>,
): NavigableElementInteractionHandlers<ElementType> {
	return generateHandlers()
}

function _structElementsEvtHandlers<
	O extends {
		[ElementType in NavigableElementType]: NavigableElementInteractionHandlers<ElementType>
	},
>(
	itemsEvtHandlers: O,
): {
	[ElementType in NavigableElementType]: NavigableElementInteractionHandlers<ElementType>
} {
	return itemsEvtHandlers
}

function navigateToFirstDescendantIn(
	children: ConcreteNavigable<NavigableElement>[],
	enteringFrom: NavigationDirection,
	handleFocusPriority?: boolean,
): NavigationResult {
	if (handleFocusPriority) {
		for (const child of children) {
			if (child.navEl.type === 'item' && child.navEl.hasFocusPriority) {
				return { type: 'focusItem', item: child.navEl }
			}
		}
	}

	for (const { navEl } of children) {
		if (navEl.type === 'item') {
			return { type: 'focusItem', item: navEl }
		}

		const descendant = triggerNavigableEvent(navEl, 'enterFrom', enteringFrom)

		if (descendant.type !== 'propagate') {
			return descendant
		}
	}

	return { type: 'propagate' }
}

export const ELEMENTS_EVENT_HANDLERS = _structElementsEvtHandlers({
	item: {
		press: (_) => {},
		longPress: (_) => {},
		interceptKeyPress: (_) => ({ type: 'propagate' }),
		focus: (_) => {},
		unfocus: (_) => {},
	},

	list: _structElementEvtHandlers('list', () => ({
		navigate(listEl, currentChild, direction) {
			const children = getChildrenOf(listEl)

			const childIndex = children.findIndex(({ navEl }) => navEl.id === currentChild.id)

			if (childIndex === -1) {
				logFatal(`Child navigable element "${currentChild.id}" was not found in its parent list "${listEl.id}"`)
			}

			switch (direction) {
				case NavigationDirection.Up:
				case NavigationDirection.Down: {
					return navigateToFirstDescendantIn(
						direction === NavigationDirection.Up
							? children.slice(0, childIndex).reverse()
							: children.slice(childIndex + 1),

						direction === NavigationDirection.Up ? NavigationDirection.Down : NavigationDirection.Up,
					)
				}

				case NavigationDirection.Left:
				case NavigationDirection.Right:
					return { type: 'propagate' }

				case NavigationDirection.Beginning:
					return navigateToFirstDescendantIn(children, NavigationDirection.Up)

				case NavigationDirection.End:
					return navigateToFirstDescendantIn(children.reverse(), NavigationDirection.Down)

				case NavigationDirection.Back:
					return { type: 'propagate' }

				case NavigationDirection.DirectFocus:
					logFatal('Unreachable: direct focus navigation on list')
			}
		},

		enterFrom(navEl, from) {
			const children = getChildrenOf(navEl)

			return navigateToFirstDescendantIn(from === NavigationDirection.Down ? children.reverse() : children, from, true)
		},

		interceptKeyPress: () => ({ type: 'propagate' }),

		focus: () => {},
		unfocus: () => {},
	})),

	row: _structElementEvtHandlers('row', () => ({
		navigate(listEl, currentChild, direction) {
			const children = getChildrenOf(listEl)

			const childIndex = children.findIndex(({ navEl }) => navEl.id === currentChild.id)

			if (childIndex === -1) {
				logFatal(`Child navigable element "${currentChild.id}" was not found in its parent row "${listEl.id}"`)
			}

			switch (direction) {
				case NavigationDirection.Left:
				case NavigationDirection.Right: {
					return navigateToFirstDescendantIn(
						direction === NavigationDirection.Left
							? children.slice(0, childIndex).reverse()
							: children.slice(childIndex + 1),

						direction === NavigationDirection.Left ? NavigationDirection.Right : NavigationDirection.Left,
					)
				}

				case NavigationDirection.Up:
				case NavigationDirection.Down:
					return { type: 'propagate' }

				case NavigationDirection.Beginning:
					return navigateToFirstDescendantIn(children, NavigationDirection.Left)

				case NavigationDirection.End:
					return navigateToFirstDescendantIn(children.reverse(), NavigationDirection.Right)

				case NavigationDirection.Back:
					return { type: 'propagate' }

				case NavigationDirection.DirectFocus:
					logFatal('Unreachable: direct focus navigation on list')
			}
		},

		enterFrom(navEl, from) {
			const children = getChildrenOf(navEl)

			return navigateToFirstDescendantIn(from === NavigationDirection.Right ? children.reverse() : children, from, true)
		},

		interceptKeyPress: () => ({ type: 'propagate' }),

		focus: () => {},
		unfocus: () => {},
	})),

	grid: _structElementEvtHandlers('grid', () => {
		function makeRows(
			gridEl: NavigableElementByType<'grid'>,
			children: ConcreteNavigable<NavigableElement>[],
		): ConcreteNavigable<NavigableElement>[][] {
			return new Array(Math.ceil(children.length / gridEl.columns))
				.fill(null)
				.map((_, i) => children.slice(i * gridEl.columns, i * gridEl.columns + gridEl.columns))
		}

		return {
			navigate(grid, currentChild, direction) {
				const children = getChildrenOf(grid)
				const rows = makeRows(grid, children)

				const childIndex = children.findIndex((child) => child.navEl.id === currentChild.id)

				if (childIndex === -1) {
					logFatal('Focused element not found in navigable row')
				}

				switch (direction) {
					case NavigationDirection.Up: {
						const rowIndex = Math.floor(childIndex / grid.columns)

						if (rowIndex === 0) {
							return { type: 'propagate' }
						}

						return navigateToFirstDescendantIn(
							rows
								.slice(0, rowIndex)
								.map((row) => row[childIndex % grid.columns])
								.reverse(),

							NavigationDirection.Down,
						)
					}

					case NavigationDirection.Down: {
						const rowIndex = Math.floor(childIndex / grid.columns)

						if (rowIndex === rows.length - 1) {
							return { type: 'propagate' }
						}

						return navigateToFirstDescendantIn(
							rows.slice(rowIndex + 1).map((row) => row[childIndex % grid.columns]),
							NavigationDirection.Down,
						)
					}

					case NavigationDirection.Left:
					case NavigationDirection.Right: {
						const isLeft = direction === NavigationDirection.Left

						const row =
							rows.find((row) => row.findIndex((child) => child.navEl.id === currentChild.id) !== -1) ??
							logFatal('Internal error: failed to find focused row in grid')

						const sliced = isLeft
							? row.slice(0, childIndex % grid.columns).reverse()
							: row.slice((childIndex % grid.columns) + 1)

						return navigateToFirstDescendantIn(sliced, isLeft ? NavigationDirection.Right : NavigationDirection.Left)
					}

					case NavigationDirection.Beginning:
					case NavigationDirection.DirectFocus:
					case NavigationDirection.End: {
						return navigateToFirstDescendantIn(
							direction === NavigationDirection.End ? children.reverse() : children,
							direction,
						)
					}

					case NavigationDirection.Back:
						return { type: 'propagate' }

					default:
						assertNever(direction)
				}
			},

			enterFrom(grid, from) {
				const children = getChildrenOf(grid)

				switch (from) {
					case NavigationDirection.Up:
					case NavigationDirection.Left:
					case NavigationDirection.Beginning:
					case NavigationDirection.DirectFocus:
						return navigateToFirstDescendantIn(children, from, true)

					case NavigationDirection.Right:
						return navigateToFirstDescendantIn(makeRows(grid, children)[0].reverse(), from, true)

					case NavigationDirection.Down:
						return navigateToFirstDescendantIn(
							makeRows(grid, children)
								.map((row) => row[0])
								.reverse(),
							from,
						)

					case NavigationDirection.End:
						return navigateToFirstDescendantIn(children.reverse(), from, true)

					case NavigationDirection.Back:
						return { type: 'propagate' }

					default:
						assertNever(from)
				}
			},

			interceptKeyPress: () => ({ type: 'propagate' }),

			focus: () => {},
			unfocus: () => {},
		}
	}),

	customContainer: {
		navigate: () => ({ type: 'propagate' }),
		enterFrom: () => ({ type: 'propagate' }),
		interceptKeyPress: () => ({ type: 'propagate' }),
		focus: () => {},
		unfocus: () => {},
	},
})

export const DATA_NAV_ATTR_NAME = 'data-nav'
export const FOCUSED_NAV_ATTR_NAME = 'data-nav-focused'

type ConcreteNavigable<ElementType extends NavigableElement> = {
	domEl: Element
	navEl: ElementType
}

export function getNavigableDOMElementById(id: string): Element | null {
	return document.querySelector(`[${DATA_NAV_ATTR_NAME}^="${id};"]`)
}

export function getNavigableElementById(id: string): ConcreteNavigable<NavigableElement> | null {
	const domEl = getNavigableDOMElementById(id)

	return domEl ? { navEl: parseNavigableDataFromElement(domEl), domEl } : null
}

export function parseNavigableDataFromElement(el: Element): NavigableElement {
	const navData =
		el.getAttribute(DATA_NAV_ATTR_NAME) ??
		logFatal(`Missing navigable data (expected attribute "${DATA_NAV_ATTR_NAME}" on HTML element)`)

	return parseNavigableElementData(navData)
}

export function getNavigableParent(
	el: ConcreteNavigable<NavigableElement>,
): ConcreteNavigable<NavigableContainer> | null {
	const parent = el.domEl.parentElement

	if (!parent) {
		return null
	}

	const parentNavDomEl = parent.closest(`[${DATA_NAV_ATTR_NAME}]`)

	if (!parentNavDomEl) {
		return null
	}

	const parentNav = parseNavigableDataFromElement(parentNavDomEl)

	if (parentNav.type === 'item') {
		logFatal(
			`Navigable "${parentNav.id}" (parent of "${el.navEl.id}") is an item ; but item cannot contain other navigables`,
		)
	}

	return { navEl: parentNav, domEl: parentNavDomEl }
}

function getNavigableAncestors(el: ConcreteNavigable<NavigableElement>): ConcreteNavigable<NavigableContainer>[] {
	const ancestors: ConcreteNavigable<NavigableContainer>[] = []
	let current = getNavigableParent(el)

	while (current) {
		ancestors.push(current)
		current = getNavigableParent(current)
	}

	return ancestors
}

export function isValidElementType(type: string): type is keyof typeof ELEMENTS_PARSER {
	return Object.hasOwn(ELEMENTS_PARSER, type)
}

function parseNavigableElementData(data: string): NavigableElement {
	const splitData = data.split(';')

	const id = splitData[0] ?? logFatal(`Missing ID in navigable data: ${data}`)
	const type = splitData[1] ?? logFatal(`Missing type in navigable data: ${data}`)

	if (!isValidElementType(type)) {
		logFatal(`Unknown element type "${type}" in navigable data: ${data}`)
	}

	const params = new Map(
		splitData.slice(2).map((segment) => {
			const sep = segment.indexOf(':')

			if (sep === -1) {
				throw new Error(`Missing separator in segment "${segment}" in navigable data: ${data}`)
			}

			return [segment.substring(0, sep), segment.substring(sep + 1)]
		}),
	)

	const navigableElt: object = parseParams(params, { ...COMMON_ELEMENTS_PROPS_PARSER, ...ELEMENTS_PARSER[type] }, type)

	// biome-ignore lint/suspicious/noExplicitAny: -
	return { ...(navigableElt as any), id, type }
}

function parseParams<V extends Record<string, ParamParser<unknown>>>(
	params: Map<string, string>,
	parsers: V,
	elementType: NavigableElementType,
): { [Key in keyof V]: Exclude<ReturnType<V[Key]>, Error> } {
	for (const paramName of params.keys()) {
		if (!Object.hasOwn(parsers, paramName)) {
			logFatal(`Unkown parameter "${paramName}" supplied for element type "${elementType}"`)
		}
	}

	const out: Record<string, unknown> = {}

	for (const [paramName, parser] of Object.entries(parsers)) {
		const parsed = parser(params.get(paramName))

		if (parsed instanceof Error) {
			logFatal(`Error for parameter "${paramName}" for element type "${elementType}": ${parsed.message}`)
		}

		out[paramName] = parsed
	}

	// biome-ignore lint/suspicious/noExplicitAny: -
	return out as any
}

type _AddFallbackNativeResult<T> = T extends (...args: infer Args) => NavigationResult
	? (...args: Args) => NavigationResult | NavigationNativeFallbackResult
	: T

type KeysOfUnion<T> = T extends T ? keyof T : never

type _AllCustomInteractionHandlers = {
	[EventType in KeysOfUnion<NavigableElementInteractionHandlers<NavigableElementType>>]: _AddFallbackNativeResult<
		Extract<NavigableElementInteractionHandlers<NavigableElementType>, { [_ in EventType]: unknown }>[EventType]
	>
}

export const ELEMENTS_CUSTOM_EVT_HANDLERS = new Map<string, Partial<_AllCustomInteractionHandlers>>()

export type NavigableElementCustomInteractionHandlers<ElementType extends NavigableElementType> = {
	[EventType in keyof NavigableElementInteractionHandlers<ElementType>]: _AddFallbackNativeResult<
		NavigableElementInteractionHandlers<ElementType>[EventType]
	>
}

// export const ELEMENTS_CUSTOM_EVT_HANDLERS = new Map<string, NavigableElementInteractionHandlers<NavigableElementType>>()

let isHandlingInteraction = false

let focusedItemId: string | null = null

export function isNavigableItem(el: ConcreteNavigable<NavigableElement>): el is ConcreteNavigable<NavigableItem> {
	return el.navEl.type === 'item'
}

export function isNavigableContainer(
	el: ConcreteNavigable<NavigableElement>,
): el is ConcreteNavigable<NavigableContainer> {
	return el.navEl.type !== 'item'
}

export function translateNavigationKey(key: string): NavigationDirection | null {
	const keys: Record<string, NavigationDirection> = {
		ArrowUp: NavigationDirection.Up,
		ArrowDown: NavigationDirection.Down,
		ArrowLeft: NavigationDirection.Left,
		ArrowRight: NavigationDirection.Right,
		Escape: NavigationDirection.Back,
		Home: NavigationDirection.Beginning,
		End: NavigationDirection.End,

		// Some aliases for keyboards with less keys
		'²': NavigationDirection.Back,
		F4: NavigationDirection.Back,
	}

	return Object.hasOwn(keys, key) ? keys[key] : null
}

export function handleKeyPress(key: KeyPress): void {
	if (isHandlingInteraction) {
		logFatal('[DATA RACE] Got a DOM event to handle while already handling one')
	}

	isHandlingInteraction = true

	try {
		__handleKeyPress(key)
	} finally {
		isHandlingInteraction = false
	}
}

function _oneShiftedList<T>(array: T[], withFirst: T): [T, T][] {
	return array.map((value, i) => [value, i === 0 ? withFirst : array[i - 1]])
}

function __handleKeyPress(key: KeyPress): void {
	if (focusedItemId === null) {
		const firstItem = findFirstFocusableItem()

		if (firstItem) {
			requestFocusOnItem(firstItem)
		}

		return
	}

	const focusedItem = getNavigableElementById(focusedItemId)

	if (!focusedItem) {
		const erroneousId = focusedItemId
		focusedItemId = null
		logFatal(`Focused item with ID "${erroneousId}" was not found`)
	}

	const { navEl: focusedNavEl } = focusedItem

	if (focusedNavEl.type !== 'item') {
		focusedItemId = null
		logFatal('Retrieved focused item is not an item but a container!')
	}

	function handleNavigationResult(result: NavigationResult): boolean {
		switch (result.type) {
			case 'focusItem':
				requestFocusOnItem(result.item)
				return true

			case 'trap':
				return true

			case 'propagate':
				return false

			default:
				assertNever(result)
		}
	}

	const interception = triggerNavigableEvent(focusedNavEl, 'interceptKeyPress', key)

	if (handleNavigationResult(interception)) {
		return
	}

	for (const [ancestor, focusedChild] of _oneShiftedList(getNavigableAncestors(focusedItem), focusedItem)) {
		const interception = triggerNavigableEvent(ancestor.navEl, 'interceptKeyPress', key, focusedChild.navEl)

		if (handleNavigationResult(interception)) {
			return
		}
	}

	if (key.key === 'Enter') {
		triggerNavigableEvent(focusedNavEl, key.longPress ? 'longPress' : 'press')
		return
	}

	const direction = translateNavigationKey(key.key)

	if (key.longPress || !direction) {
		return
	}

	// trigger navigation in parent
	let parent = getNavigableParent(focusedItem)

	if (!parent) {
		getSetupOptions().log(LogLevel.Warn, `Navigable item "${focusedItem.navEl.id}" does not have a parent!`)
		return
	}

	let current: ConcreteNavigable<NavigableElement> = focusedItem

	while (parent) {
		const infos = { parent, current }
		const result = triggerNavigableEvent(infos.parent.navEl, 'navigate', infos.current.navEl, direction)

		if (result.type === 'focusItem') {
			requestFocusOnItem(result.item)
			break
		}

		if (result.type === 'trap') {
			break
		}

		current = parent
		parent = getNavigableParent(parent)
	}
}

export function findFirstFocusableItem(): NavigableItem | null {
	// TODO: optimize

	for (const htmlEl of document.querySelectorAll(`[${DATA_NAV_ATTR_NAME}]`)) {
		const navEl = parseNavigableDataFromElement(htmlEl)

		if (navEl.type === 'item') {
			return navEl
		}
	}

	getSetupOptions().log(LogLevel.Warn, 'No navigable item found in page!')
	return null
}

let isHandlingFocusRequest = false

export function requestFocusOnItem(navEl: NavigableItem): void {
	if (isHandlingFocusRequest) {
		logFatal('[DATA RACE] Got a request focus while already handling one')
	}

	if (navEl.id === focusedItemId) {
		return
	}

	isHandlingFocusRequest = true

	const runHandlers: (() => void)[] = []

	const domEl =
		getNavigableDOMElementById(navEl.id) ??
		logFatal(`Internal error: newly-focused item "${navEl.id}" does not have a DOM element`)

	const previouslyFocused = focusedItemId !== null ? getNavigableElementById(focusedItemId) : null

	const previouslyFocusedAncestors = previouslyFocused ? getNavigableAncestors(previouslyFocused) : []
	const newlyFocusedAncestors = getNavigableAncestors({ navEl, domEl })

	if (previouslyFocused) {
		previouslyFocused.domEl.removeAttribute(FOCUSED_NAV_ATTR_NAME)

		runHandlers.push(() => triggerNavigableEvent(previouslyFocused.navEl, 'unfocus'))

		const unfocusedAncestors = previouslyFocusedAncestors.filter(
			(el) => !newlyFocusedAncestors.find((c) => c.navEl.id === el.navEl.id),
		)

		for (const [ancestor, ancestorChild] of _oneShiftedList(unfocusedAncestors, previouslyFocused)) {
			runHandlers.push(() => triggerNavigableEvent(ancestor.navEl, 'unfocus', ancestorChild.navEl))
		}
	}

	domEl.setAttribute(FOCUSED_NAV_ATTR_NAME, 'true')

	runHandlers.push(() => triggerNavigableEvent(navEl, 'focus'))

	const focusedAncestors = newlyFocusedAncestors.filter(
		(el) => !previouslyFocusedAncestors.find((c) => c.navEl.id === el.navEl.id),
	)

	const newlyFocused: ConcreteNavigable<NavigableElement> = { navEl, domEl }

	for (const [ancestor, ancestorChild] of _oneShiftedList(focusedAncestors, newlyFocused)) {
		runHandlers.push(() => triggerNavigableEvent(ancestor.navEl, 'focus', ancestorChild.navEl))
	}

	focusedItemId = navEl.id

	if (![newlyFocused].concat(focusedAncestors).find((el) => el.navEl.disableScroll)) {
		domEl.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'nearest' })
	}

	for (const handler of runHandlers) {
		try {
			handler()
		} finally {
		}
	}

	isHandlingFocusRequest = false
}

export function requestFocusOnElement(navEl: NavigableElement): void {
	if (navEl.type === 'item') {
		requestFocusOnItem(navEl)
		return
	}

	function tryRun<T>(handler: () => T): T {
		try {
			return handler()
		} finally {
			isHandlingInteraction = false
		}
	}

	const result = tryRun(() => triggerNavigableEvent(navEl, 'enterFrom', NavigationDirection.DirectFocus))

	if (result.type === 'focusItem') {
		isHandlingInteraction = false
		requestFocusOnItem(result.item)
	} else {
		getSetupOptions().log(
			LogLevel.Warn,
			'Failed to request focus on provided container as it returned no item to focus on',
		)
	}
}

export function requestFocusById(navId: string): void {
	const el = getNavigableElementById(navId) ?? logFatal(`Requested focus on non-existing element "${navId}"`)

	requestFocusOnElement(el.navEl)
}

export function requestFocusToDOMElement(domEl: Element): void {
	const navEl = parseNavigableDataFromElement(domEl) ?? logFatal('Requested focus on invalid navigable DOM element')

	requestFocusOnElement(navEl)
}

export function getFocusedItemId(): string | null {
	return focusedItemId
}

type Merge<T> = {
	[K in keyof T]: T[K]
}

export type OptionalUndefined<
	T,
	Props extends keyof T = keyof T,
	OptionsProps extends keyof T = Props extends keyof T ? (undefined extends T[Props] ? Props : never) : never,
> = Merge<
	{
		[K in OptionsProps]?: T[K]
	} & {
		[K in Exclude<keyof T, OptionsProps>]: T[K]
	}
>

export function parseConcreteNavigable(domEl: Element): ConcreteNavigable<NavigableElement> {
	return { domEl, navEl: parseNavigableDataFromElement(domEl) }
}

export function getChildrenOfElement(el: Element, navEl: NavigableElement): ConcreteNavigable<NavigableElement>[] {
	const childrenEl = Array.from(
		el.querySelectorAll(
			// Select all DOM elements with the navigable attribute
			// ...which are not descendants of another navigable
			//    themselves descendants of the parent element we're starting from
			//
			// Result: select all direct descendent navigable elements
			`[${DATA_NAV_ATTR_NAME}]:not([${DATA_NAV_ATTR_NAME}^="${navEl.id};"] [${DATA_NAV_ATTR_NAME}] *)`,
		),
	)

	return childrenEl.map(parseConcreteNavigable)
}

export function getChildrenOf(navEl: NavigableContainer): ConcreteNavigable<NavigableElement>[] {
	const domEl =
		getNavigableDOMElementById(navEl.id) ?? logFatal(`No DOM element found for container element "${navEl.id}"`)

	return getChildrenOfElement(domEl, navEl)
}

// biome-ignore lint/suspicious/noExplicitAny: required for this type ('unknown' wouldn't work)
type OmitFirstArg<F> = F extends (x: any, ...args: infer P) => unknown ? P : never

// NOTE: we redefine the `ReturnType` utility here as the native one strangely doesn't work below
// biome-ignore lint/suspicious/noExplicitAny: required for this type ('unknown' wouldn't work)
type ReturnType<F> = F extends (...args: any[]) => infer U ? U : never

export function triggerNavigableEvent<
	ElementType extends NavigableElementType,
	EventType extends keyof NavigableElementInteractionHandlers<ElementType>,
>(
	navEl: NavigableElementByType<ElementType>,
	event: EventType,
	...params: OmitFirstArg<NavigableElementInteractionHandlers<ElementType>[EventType]>
): ReturnType<NavigableElementInteractionHandlers<ElementType>[EventType]> {
	const customHandler = ELEMENTS_CUSTOM_EVT_HANDLERS.get(navEl.id)?.[event as keyof _AllCustomInteractionHandlers]

	// biome-ignore lint/suspicious/noExplicitAny: -
	const nativeHandler = (ELEMENTS_EVENT_HANDLERS[navEl.type] as any)[event as any]

	if (customHandler) {
		// biome-ignore lint/suspicious/noExplicitAny: -
		const ret = customHandler(navEl as any, ...(params as any as [never]))

		if (ret === undefined) {
			// biome-ignore lint/suspicious/noExplicitAny: -
			return undefined as any
		}

		switch (ret.type) {
			case 'focusItem':
			case 'propagate':
			case 'trap':
				// biome-ignore lint/suspicious/noExplicitAny: -
				return ret as any

			case 'native':
				return nativeHandler(navEl, ...params)

			default:
				assertNever(ret)
		}
	}

	return nativeHandler(navEl, ...params)
}

let idCounter = 0

export function generateNavigableElementId(): string {
	idCounter += 1
	return idCounter.toString()
}

export function registerNavigableElementHandlers<ElementType extends NavigableElementType>(
	element: NavigableElementByType<ElementType>,
	eventHandlers: Partial<NavigableElementCustomInteractionHandlers<ElementType>> | null,
): void {
	if (eventHandlers !== null) {
		ELEMENTS_CUSTOM_EVT_HANDLERS.set(
			element.id,
			// biome-ignore lint/suspicious/noExplicitAny: TypeScript limitations
			eventHandlers as any,
		)
	}
}

export function updateNavigableElementHandlers<ElementType extends NavigableElementType>(
	element: NavigableElementByType<ElementType>,
	eventHandlers: Partial<NavigableElementCustomInteractionHandlers<ElementType>> | null,
): void {
	if (eventHandlers !== null) {
		ELEMENTS_CUSTOM_EVT_HANDLERS.set(
			element.id,
			// biome-ignore lint/suspicious/noExplicitAny: TypeScript limitations
			eventHandlers as any,
		)
	} else {
		ELEMENTS_CUSTOM_EVT_HANDLERS.delete(element.id)
	}
}

export function unregisterNavigableElementHandlers<ElementType extends NavigableElementType>(
	element: NavigableElementByType<ElementType>,
): void {
	ELEMENTS_CUSTOM_EVT_HANDLERS.delete(element.id)

	if (focusedItemId === element.id) {
		getSetupOptions().log(
			LogLevel.Warn,
			`Navigable item with ID '${element.id}' has been removed while still being focused`,
		)

		focusedItemId = null
	}
}

export class HTMLNavigableElementWrapper extends HTMLElement {}

export function encodeNavigableElement(nav: OptionalUndefined<NavigableElement>): string {
	const params = Object.entries(nav)
		.filter(([key, _]) => key !== 'id' && key !== 'type')
		.map(([key, value]) => `${key}:${value}`)
		.join(';')

	return `${nav.id};${nav.type}${params.length > 0 ? `;${params}` : ''}`
}

export function navigableElementAttrs(nav: OptionalUndefined<NavigableElement>): Record<string, string> {
	return { [DATA_NAV_ATTR_NAME]: encodeNavigableElement(nav) }
}
