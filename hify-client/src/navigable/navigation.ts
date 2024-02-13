import { getContext, setContext } from 'svelte'
import { get, writable } from 'svelte/store'
import { logWarn } from '../stores/debugger'
import { KeyPressHandling, handleInput, registerLongPressableKeys } from './input-manager'

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
	hasFocusPriority: boolean | null
	onFocusChangeCallback?: OnFocusChangeCallback | null
}

export type Props<N extends NavigableCommon> = N['props']

// NOTE: This is askin to an empty type
export type NoProp = Record<never, never>

export abstract class NavigableCommon<P = NoProp> {
	readonly parent: NavigableContainer<unknown>
	readonly identity: symbol
	readonly page: NavigablePage
	readonly id: string

	protected focused = writable(false)

	constructor(parent: NavigableContainer<unknown> | symbol, protected _props: NavigableCommonProps & P) {
		if (!isValidNavigable(this)) {
			throw new Error(
				'Invalid navigable detected ; must be an instance of either "NavigableContainer" or "NavigableItem"',
			)
		}

		this.id = generateNavigableId()

		if (!(parent instanceof NavigableContainer)) {
			if (parent !== PAGE_CTR_TOKEN) {
				throw new Error('Invalid page construction token provided!')
			}

			// biome-ignore lint/suspicious/noExplicitAny: required here
			this.parent = undefined as any
			// biome-ignore lint/suspicious/noExplicitAny: required here
			this.identity = undefined as any
			// biome-ignore lint/suspicious/noExplicitAny: required here
			this.page = undefined as any

			return
		}

		this.parent = parent
		this.identity = parent.identity
		this.page = parent.page

		this.page.registerNavigable(this)
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

export abstract class NavigableContainer<P = NoProp> extends NavigableCommon<P> {
	protected getFocusPriority(): Navigable | null {
		return this.children().find((item) => item.props.hasFocusPriority === true) ?? null
	}

	abstract navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem<unknown> | null

	canHandleAction(_: NavigationAction): boolean {
		return false
	}

	handleAction(_: NavigationAction): NavigableItem<unknown> | null {
		throw new Error('This navigable container does not support actions')
	}

	children(): Navigable[] {
		const children: Navigable[] = []

		visitNavigableChildren(this.id, this.page, (nav) => {
			children.push(nav)
		})

		// NOTE: Just a quick assertion to ensure there is no bug here
		if (children.find((child) => child.id === this.id)) {
			throw new Error('Self item found in its children!')
		}

		return children
	}

	navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem<unknown> | null {
		return this.children()[0]?.navigateToFirstItemDown(from) ?? null
	}

	navigateToLastItem(): NavigableItem<unknown> | null {
		for (const child of this.children()) {
			const target = child.navigateToLastItem()

			if (target) {
				return target
			}
		}

		return null
	}

	requestFocus(): boolean {
		let result = false

		visitNavigableChildren(this.id, this.page, (nav) => {
			result = nav.requestFocus()
			return true
		})

		return result
	}
}

export abstract class NavigableItem<P = NoProp> extends NavigableCommon<P> {
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
			throw new Error(`Item's underlying element is not an ${HTMLNavigableItemWrapperElement.name}`)
		}

		if (!(el.children.length && el.children[0] instanceof HTMLElement)) {
			return console.warn('Failed to scroll to element has it does not have a valid child element')
		}

		if (el.children.length > 0) {
			el.children[0].scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'nearest' })
		} else {
			console.warn('Navigable item has no children ; cannot scroll into view')
		}
	}

	wasDestroyed(): boolean {
		return !this.page.containsItem(this)
	}
}

class NavigablePage extends NavigableContainer {
	override readonly identity: symbol
	override readonly page: NavigablePage
	override readonly parent: NavigableContainer

	// TODO: garbage collection when elements are removed (remove on 'onDestroy' event?)
	protected readonly navigables: Map<string, Navigable>

	readonly priorityFocusables: NavigableItem<unknown>[] = []
	readonly ordered = false

	constructor(
		private readonly onRequestFocus: (item: NavigableItem<unknown>) => void,
		private readonly onRequestUnfocus: () => void,
		private readonly getFocusedItem: () => NavigableItem<unknown> | null,
	) {
		super(PAGE_CTR_TOKEN, { hasFocusPriority: null })

		this.identity = Symbol()
		this.page = this
		this.parent = this
		this.navigables = new Map()
	}

	override canHandleAction(_: NavigationAction): boolean {
		return false
	}

	override handleAction(_: NavigationAction): NavigableItem<unknown> | null {
		throw new Error('Tried to make the navigable page component handle an action')
	}

	registerNavigable(nav: Navigable): void {
		this.navigables.set(nav.id, nav)
	}

	getNavigableFromId(navId: string): Navigable {
		const nav = this.navigables.get(navId)

		if (nav === undefined) {
			throw new Error('Navigable element does not have an identity in the ancestor page!')
		}

		return nav
	}

	getNavigableFromItemElement(el: Element): Navigable {
		if (!(el instanceof HTMLNavigableItemWrapperElement)) {
			throw new Error('Cannot get the navigable from a non-item wrapper element')
		}

		const navId = el.getAttribute(NAV_ITEM_ID_ATTR_NAME)

		if (navId === null) {
			throw new Error('Navigable ID is missing on item wrapper element!')
		}

		return this.getNavigableFromId(navId)
	}

	containsItem(item: Navigable): boolean {
		return this.navigables.has(item.id)
	}

	navigate(_: Navigable, __: NavigationDirection): NavigableItem<unknown> | null {
		return null
	}

	requestPageFocus(item: NavigableItem<unknown>): void {
		if (item.parent.page !== this) {
			throw new Error("Cannot request focus for an element that isn't part of the same page")
		}

		this.onRequestFocus(item)
	}

	unfocus(): void {
		const focused = this.focusedItem()

		if (focused === null) {
			return logWarn('No item focused, cannot unfocus')
		}

		this.onRequestUnfocus()
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
		if (getContext(NAVIGABLE_ITEM_DETECTION_CTX)) {
			throw new Error('Cannot use a navigable inside an item')
		}

		setContext(NAVIGABLE_ITEM_DETECTION_CTX, true)
	}

	const nav = getContext(NAVIGATION_CTX)

	if (nav === null || nav === undefined) {
		throw new Error('No parent navigable found in the current context')
	}

	if (!(nav instanceof NavigableContainer || nav instanceof NavigablePage)) {
		throw new Error('Context does not contain a navigable value')
	}

	return nav
}

export function setChildrenNavigable(nav: NavigableContainer) {
	setContext(NAVIGATION_CTX, nav)
}

export function usePageNavigator(): NavigableContainer {
	const page = new NavigablePage(_requestFocus, _requestUnfocus, () => get(navState)?.focused ?? null)

	navState.update((state) => {
		state?.focused?.onUnfocus()
		return { page, focused: null }
	})

	setChildrenNavigable(page)

	return page
}

export function isValidNavigable(nav: NavigableCommon<unknown>): nav is Navigable {
	return nav instanceof NavigableContainer || nav instanceof NavigableItem
}

function visitNavigableChildren(
	navId: string,
	page: NavigablePage,
	inspector: (nav: Navigable) => void | boolean,
): void | boolean {
	const startFrom = document.evaluate(
		`//comment()[. = ' @start-nav: ${navId} ']`,
		document,
		null,
		XPathResult.FIRST_ORDERED_NODE_TYPE,
		null,
	).singleNodeValue

	if (startFrom === null) {
		throw new Error(`Navigable with the provided ID "${navId}" was not found!`)
	}

	const visitNode = (node: Node): [boolean | void, number] => {
		if (node.nodeType === Node.COMMENT_NODE) {
			const match = node.textContent?.match(/^ @start-nav: (.*) $/)

			if (!match) {
				return [false, 0]
			}

			const result = inspector(page.getNavigableFromId(match[1]))

			let next: Node | null = node

			let i = 0

			for (;;) {
				next = next.nextSibling
				i += 1

				if (next === null) {
					throw new Error(`Closing comment not found for navigable with ID "${match[1]}"!`)
				}

				if (next.nodeType === Node.COMMENT_NODE && next.textContent === ` @end-nav: ${match[1]} `) {
					return [result, i]
				}
			}
		}

		if (node.nodeType !== Node.ELEMENT_NODE) {
			return [false, 0]
		}

		if (!(node instanceof Element)) {
			throw new Error('Assertion error: element node is not an instance of Element')
		}

		if (
			node.tagName.toLowerCase() === ITEM_WRAPPER_ELEMENT_TAG_NAME &&
			node.getAttribute(NAV_ITEM_ID_ATTR_NAME) !== JUST_FOR_STYLE_ITEM_ID
		) {
			return [inspector(page.getNavigableFromItemElement(node)), 0]
		}

		const children = Array.from(node.childNodes)

		for (let i = 0; i < children.length; i++) {
			const [result, skip] = visitNode(children[i])

			if (result === true) {
				return [result, skip]
			}

			i += skip
		}

		return [false, 0]
	}

	let curr: Node | null = startFrom

	for (;;) {
		curr = curr.nextSibling

		if (curr === null) {
			throw new Error('Closing comment not found for navigable!')
		}

		if (curr.nodeType === Node.COMMENT_NODE && curr.textContent === ` @end-nav: ${navId} `) {
			return
		}

		const [result, skip] = visitNode(curr)

		if (result === true) {
			return true
		}

		for (let i = 0; i < skip; i++) {
			curr = curr.nextSibling

			if (curr === null) {
				throw new Error('Found inexistant node while skipping')
			}
		}
	}
}

const NAVIGABLE_ID_CHARS = 'abcdefghijklmnopqrstuvwxyz01234567890'

function generateNavigableId(): string {
	let out = ''

	for (let i = 0; i < 16; i++) {
		if (i > 0 && i % 4 === 0) {
			out += '-'
		}

		out += NAVIGABLE_ID_CHARS.charAt(Math.floor(Math.random() * NAVIGABLE_ID_CHARS.length))
	}

	return out
}

function handleKeyboardEvent(key: string, long: boolean): void {
	const state = get(navState)

	if (!state) {
		return
	}

	let __current = state.focused

	if (__current) {
		if (__current.identity !== state.page.identity) {
			console.warn('Previously-focused element has a different identity than the current page, removing focus')
			__current.onUnfocus()
			__current = null
		} else if (__current.wasDestroyed()) {
			console.warn('Previously-focused element was destroyed, removing focus')
			__current.onUnfocus()
			__current = null
		}
	}

	let currentJustFocused = false

	if (!__current) {
		__current = state.page.navigateToFirstItemDown(NavigationComingFrom.Above)

		if (!__current) {
			console.warn('No navigable item in this page')
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
		case 'ArrowDown': {
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
		}

		case 'Enter':
		case 'Escape': {
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
		}

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

	let current: NavigableContainer<unknown> = item.parent

	while (!(current instanceof NavigablePage)) {
		out.push(current)
		current = current.parent
	}

	return out
}

function _checkItemValidity(item: NavigableItem<unknown>, page: NavigablePage): boolean {
	if (item.identity !== page.identity) {
		console.warn('Previously-focused element has a different identity than the current page, removing focus')
		item.onUnfocus()
		return false
	}

	if (item.wasDestroyed()) {
		console.warn('Previously-focused element was destroyed, removing focus')
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

function _requestUnfocus(): void {
	navState.update((state) => (state ? _generateUpdatedNavState(state.focused, null, state.page) : state))
}

function _generateUpdatedNavState(
	oldFocused: NavigableItem<unknown> | null,
	newFocused: NavigableItem<unknown> | null,
	page: NavigablePage,
): NavState {
	// TODO: handle if oldFocused === newFocused

	if (oldFocused) {
		oldFocused.onUnfocus()
		_propagateFocusChangeEvent(oldFocused, false)
	}

	if (newFocused) {
		newFocused.scrollTo()
		newFocused.onFocus()
		_propagateFocusChangeEvent(newFocused, true)
	}

	return { page, focused: newFocused }
}

function _propagateFocusChangeEvent(item: NavigableItem<unknown>, focused: boolean): void {
	for (const subItem of _getItemChain(item)) {
		subItem.props.onFocusChangeCallback?.(focused)
	}
}

export type RequestFocus = () => boolean

export type Navigable = NavigableContainer<unknown> | NavigableItem<unknown>

const NAVIGATION_CTX = Symbol()
const NAVIGABLE_ITEM_DETECTION_CTX = Symbol()
const PAGE_CTR_TOKEN = Symbol()

type NavState = {
	page: NavigablePage
	focused: NavigableItem<unknown> | null
}

const navState = writable<NavState | null>(null)

export class HTMLNavigableItemWrapperElement extends HTMLElement {}

export const ITEM_WRAPPER_ELEMENT_TAG_NAME = 'navigable-item-wrapper'

const itemWrapperInPlace = window.customElements.get(ITEM_WRAPPER_ELEMENT_TAG_NAME)

if (!itemWrapperInPlace) {
	window.customElements.define(ITEM_WRAPPER_ELEMENT_TAG_NAME, HTMLNavigableItemWrapperElement)
} else if (itemWrapperInPlace.name !== HTMLNavigableItemWrapperElement.name) {
	throw new Error('An invalid item wrapper element is already in place')
}

export const NAV_ITEM_ID_ATTR_NAME = 'data-navigable-item-id'
export const JUST_FOR_STYLE_ITEM_ID = ':just_for_style'

// Support long-press for "Enter" key
registerLongPressableKeys('Enter')

handleInput(handleKeyboardEvent)
