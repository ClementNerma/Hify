import {
	Navigable,
	NavigableContainer,
	NavigableItem,
	NavigationComingFrom,
	NavigationDirection,
	NoProp,
} from '../../navigation'

export class NavigableList<P = NoProp> extends NavigableContainer<NavigableListProps & P> {
	private itemsBeforeLastLazyLoading = 0

	private _lazyLoading(items: Navigable[]) {
		if (!this.props.lazyLoader) {
			return
		}

		if (this.itemsBeforeLastLazyLoading !== items.length) {
			this.itemsBeforeLastLazyLoading = items.length
			this.props.lazyLoader()
		}
	}

	navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem<unknown> | null {
		const items = this.children()

		const rowIndex = items.indexOf(focusedChild)

		if (rowIndex === -1) {
			throw new Error('Focused element not found in navigable list')
		}

		switch (direction) {
			case NavigationDirection.Up:
				for (const rowItem of items.slice(0, rowIndex).reverse()) {
					const item = rowItem.navigateToFirstItemDown(NavigationComingFrom.Below)

					if (item) {
						return item
					}
				}

				if (this.props.trapped) {
					return null
				}

				break

			case NavigationDirection.Left:
			case NavigationDirection.Right:
				break

			case NavigationDirection.Down: {
				const items = this.children()

				// Default to 3 means that when we enter one of the grid's last three lines,
				// lazy loading will be triggered. This avoids sluggish response by lazy loading
				// early, before the user is actually at the end of the grid.
				const distanceBeforeLazyLoading = this._props.distanceBeforeLazyLoading ?? 3

				// Required to trigger lazy loader when either:
				// * We navigate to the last row from the above one
				// * We navigate to the last row from below
				if (rowIndex >= items.length - 1 - distanceBeforeLazyLoading) {
					this._lazyLoading(items)
				}

				if (rowIndex === items.length - 1) {
					break
				}

				for (const rowItem of items.slice(rowIndex + 1)) {
					const item = rowItem.navigateToFirstItemDown(NavigationComingFrom.Above)

					if (item) {
						return item.navigateToFirstItemDown(NavigationComingFrom.Above)
					}
				}

				if (this.props.trapped) {
					return null
				}
			}
		}

		return this.parent.navigate(this, direction)
	}

	override navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem<unknown> | null {
		const items = this.children()

		let tries: Navigable[]

		switch (from) {
			case NavigationComingFrom.Left:
			case NavigationComingFrom.Right: {
				const prio = this.getFocusPriority()

				if (prio) {
					return prio.navigateToFirstItemDown(from)
				}

				tries = items
				break
			}

			case NavigationComingFrom.Above:
				tries = items
				break

			case NavigationComingFrom.Below:
				tries = [...items].reverse()
				this._lazyLoading(items)
				break
		}

		for (const child of tries) {
			const item = child.navigateToFirstItemDown(from)

			if (item) {
				return item
			}
		}

		return null
	}

	override navigateToLastItem(): NavigableItem<unknown> | null {
		this._lazyLoading(this.children())

		return super.navigateToLastItem()
	}
}

type NavigableListProps = {
	trapped?: boolean
	lazyLoader?: () => void
	distanceBeforeLazyLoading?: number
}
