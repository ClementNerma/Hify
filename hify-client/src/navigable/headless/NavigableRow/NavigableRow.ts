import {
	Navigable,
	NavigableContainer,
	NavigableItem,
	NavigationComingFrom,
	NavigationDirection,
	NoProp,
} from '../../navigation'

export class NavigableRow<P = NoProp> extends NavigableContainer<P> {
	navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem<unknown> | null {
		const items = this.children()

		const colIndex = items.indexOf(focusedChild)

		if (colIndex === -1) {
			throw new Error('Focused element not found in navigable row')
		}

		switch (direction) {
			case NavigationDirection.Up:
				return this.parent.navigate(this, NavigationDirection.Up)

			case NavigationDirection.Left:
				for (const colItem of items.slice(0, colIndex).reverse()) {
					const item = colItem.navigateToFirstItemDown(NavigationComingFrom.Right)

					if (item) {
						return item
					}
				}

				return this.parent.navigate(this, NavigationDirection.Right)

			case NavigationDirection.Right:
				for (const colItem of items.slice(colIndex + 1)) {
					const item = colItem.navigateToFirstItemDown(NavigationComingFrom.Left)

					if (item) {
						return item
					}
				}

				return this.parent.navigate(this, NavigationDirection.Left)

			case NavigationDirection.Down:
				return this.parent.navigate(this, NavigationDirection.Down)
		}
	}

	override navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem<unknown> | null {
		const items = this.children()

		let tries: Navigable[]

		switch (from) {
			case NavigationComingFrom.Above:
			case NavigationComingFrom.Below: {
				const prio = this.getFocusPriority()

				if (prio) {
					return prio.navigateToFirstItemDown(from)
				}

				tries = items
				break
			}

			case NavigationComingFrom.Left:
				tries = items
				break

			case NavigationComingFrom.Right:
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
