import {
  Navigable,
  NavigableItem,
  NavigationComingFrom,
  NavigationDirection,
  NavigableArrayContainer,
} from '../navigation'

export class NavigableList extends NavigableArrayContainer {
  navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem | null {
    const rowIndex = this.items.indexOf(focusedChild)

    if (rowIndex === -1) {
      throw new Error('Focused element not found in navigable list')
    }

    switch (direction) {
      case NavigationDirection.Up:
        for (const rowItem of this.items.slice(0, rowIndex).reverse()) {
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
        for (const rowItem of this.items.slice(rowIndex + 1)) {
          const item = rowItem.navigateToFirstItemDown(NavigationComingFrom.Above)

          if (item) {
            return item
          }
        }

        return this.parent.navigate(this, NavigationDirection.Down)
    }
  }

  navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem | null {
    let tries: Navigable[]

    switch (from) {
      case NavigationComingFrom.Left:
      case NavigationComingFrom.Right:
        const prio = this.getFocusPriority()

        if (prio) {
          return prio.navigateToFirstItemDown(from)
        }

        tries = this.items
        break

      case NavigationComingFrom.Above:
        tries = this.items
        break

      case NavigationComingFrom.Below:
        tries = [...this.items].reverse()
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
