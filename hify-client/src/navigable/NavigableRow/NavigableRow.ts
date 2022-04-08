import {
  Navigable,
  NavigableItem,
  NavigationComingFrom,
  NavigationDirection,
  NavigableArrayContainer,
} from '../navigation'

export class NavigableRow extends NavigableArrayContainer {
  navigate(focusedChild: Navigable, direction: NavigationDirection, long: boolean): NavigableItem | null {
    const colIndex = this.items.indexOf(focusedChild)

    if (colIndex === -1) {
      throw new Error('Focused element not found in navigable row')
    }

    switch (direction) {
      case NavigationDirection.Up:
        return this.parent.navigate(this, NavigationDirection.Up, long)

      case NavigationDirection.Left: {
        const sliced = this.items.slice(0, colIndex)

        for (const colItem of long ? sliced : sliced.reverse()) {
          const item = colItem.navigateToFirstItemDown(NavigationComingFrom.Right)

          if (item) {
            return item
          }
        }

        return this.parent.navigate(this, NavigationDirection.Right, long)
      }

      case NavigationDirection.Right: {
        const sliced = this.items.slice(colIndex + 1)

        for (const colItem of long ? sliced.reverse() : sliced) {
          const item = colItem.navigateToFirstItemDown(NavigationComingFrom.Left)

          if (item) {
            return item
          }
        }

        return this.parent.navigate(this, NavigationDirection.Left, long)
      }

      case NavigationDirection.Down:
        return this.parent.navigate(this, NavigationDirection.Down, long)
    }
  }

  navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem | null {
    let tries: Navigable[]

    switch (from) {
      case NavigationComingFrom.Above:
      case NavigationComingFrom.Below:
        const prio = this.getFocusPriority()

        if (prio) {
          return prio.navigateToFirstItemDown(from)
        }

        tries = this.items
        break

      case NavigationComingFrom.Left:
        tries = this.items
        break

      case NavigationComingFrom.Right:
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
