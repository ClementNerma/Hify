import type { App, Directive, Plugin } from 'vue'
import {
	DATA_NAV_ATTR_NAME,
	encodeNavigableElement,
	generateNavigableElementId,
	type NavigableElement,
	type NavigableElementByType,
	type NavigableElementInteractionHandlers,
	type NavigableElementType,
	type OptionalUndefined,
	parseNavigableDataFromElement,
	registerNavigableElementHandlers,
	type SetupNavigableOptions,
	setupNavigable,
	unregisterNavigableElementHandlers,
	updateNavigableElementHandlers,
} from '..'

export type NavDirectiveParamsByElementType<ElementType extends NavigableElementType> = OptionalUndefined<
	Omit<NavigableElementByType<ElementType>, 'id'>
> & {
	'@on'?: Partial<NavigableElementInteractionHandlers<ElementType>>
}

export function setupVueNavigable(app: App<Element>, options: SetupNavigableOptions): void {
	setupNavigable(options)

	app.directive('nav', vNav)
	app.directive('nav-item', vNavItem)
	app.directive('nav-column', vNavColumn)
	app.directive('nav-row', vNavRow)
	app.directive('nav-grid', vNavGrid)
	app.directive('nav-custom-container', vNavCustomContainer)
}

function createVueNavDirective<T>(
	withDataTransformer: (data: T) => NavDirectiveParamsByElementType<NavigableElementType>,
) {
	return {
		mounted(el, { value }) {
			const { '@on': when, ...params } = withDataTransformer(value)

			const navEl = {
				...params,
				id: generateNavigableElementId(),
			} as NavigableElement

			el.setAttribute(DATA_NAV_ATTR_NAME, encodeNavigableElement(navEl))

			registerNavigableElementHandlers(navEl, when ?? null)
		},

		beforeUpdate(el, { value }) {
			const { '@on': when, ...params } = withDataTransformer(value)

			// TODO: optimize
			const navEl = parseNavigableDataFromElement(el)

			el.setAttribute(DATA_NAV_ATTR_NAME, encodeNavigableElement({ ...params, id: navEl.id }))

			updateNavigableElementHandlers(navEl, when ?? null)
		},

		beforeUnmount(el) {
			// TODO: optimize
			const navEl = parseNavigableDataFromElement(el)

			unregisterNavigableElementHandlers(navEl)
		},
	} satisfies Directive<HTMLElement, T>
}

export const vNav = createVueNavDirective<NavDirectiveParamsByElementType<NavigableElementType>>((data) => data)

function createVueNavDirectiveForElement<ElementType extends NavigableElementType>(elementType: ElementType) {
	return createVueNavDirective<Omit<NavDirectiveParamsByElementType<ElementType>, 'type'>>(
		(data) =>
			({
				...data,
				type: elementType,
				// biome-ignore lint/suspicious/noExplicitAny: <explanation>
			}) as any,
	)
}

export const vNavItem = createVueNavDirectiveForElement('item')
export const vNavColumn = createVueNavDirectiveForElement('column')
export const vNavRow = createVueNavDirectiveForElement('row')
export const vNavGrid = createVueNavDirectiveForElement('grid')
export const vNavCustomContainer = createVueNavDirectiveForElement('customContainer')

export const navigablePlugin = {
	install(app, navOptions) {
		setupVueNavigable(app, navOptions)
	},
} satisfies Plugin<[SetupNavigableOptions]>

declare module 'vue' {
	export interface ComponentCustomProperties {
		vNav: typeof vNav
		vNavItem: typeof vNavItem
		vNavColumn: typeof vNavColumn
		vNavRow: typeof vNavRow
		vNavGrid: typeof vNavGrid
		vNavCustomContainer: typeof vNavCustomContainer
	}
}
