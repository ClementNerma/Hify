import {
	Navigable,
	NavigableContainer,
	NavigableItem,
	NavigationComingFrom,
	NavigationDirection,
	NoProp,
} from '../../navigation'

export class NavigableList<P = NoProp> extends NavigableContainer<P> {
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

				return this.parent.navigate(this, NavigationDirection.Up)

			case NavigationDirection.Left:
				return this.parent.navigate(this, NavigationDirection.Left)

			case NavigationDirection.Right:
				return this.parent.navigate(this, NavigationDirection.Right)

			case NavigationDirection.Down:
				for (const rowItem of items.slice(rowIndex + 1)) {
					const item = rowItem.navigateToFirstItemDown(NavigationComingFrom.Above)

					if (item) {
						return item
					}
				}

				return this.parent.navigate(this, NavigationDirection.Down)
		}
	}

	override navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem<unknown> | null {
		const items = this.children()

		let tries: Navigable[]

		switch (from) {
			case NavigationComingFrom.Left:
			case NavigationComingFrom.Right:
				const prio = this.getFocusPriority()

				if (prio) {
					return prio.navigateToFirstItemDown(from)
				}

				tries = items
				break

			case NavigationComingFrom.Above:
				tries = items
				break

			case NavigationComingFrom.Below:
				tries = [...items].reverse()
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
}
