import {
  Navigable,
  NavigableItem,
  NavigationComingFrom,
  NavigationDirection,
  NavigableArrayContainer,
} from '../navigation'

export class NavigableRow extends NavigableArrayContainer {
  private readonly columns: Navigable[] = []

  append(navigable: Navigable) {
    this.columns.push(navigable)
  }

  hasChild(child: Navigable): boolean {
    return this.columns.indexOf(child) !== -1
  }

  remove(child: Navigable): void {
    const index = this.columns.indexOf(child)

    if (index === -1) {
      throw new Error('Tried to remove unknown child from navigable row')
    }

    this.columns.splice(index, 1)
  }

  navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem | null {
    const colIndex = this.columns.indexOf(focusedChild)

    if (colIndex === -1) {
      throw new Error('Focused element not found in navigable row')
    }

    switch (direction) {
      case NavigationDirection.Up:
        return this.parent.navigate(this, NavigationDirection.Up)

      case NavigationDirection.Left:
        for (const colItem of [...this.columns.slice(0, colIndex)].reverse()) {
          const item = colItem.navigateToFirstItemDown(NavigationComingFrom.Right)

          if (item) {
            return item
          }
        }

        return this.parent.navigate(this, NavigationDirection.Right)

      case NavigationDirection.Right:
        for (const colItem of this.columns.slice(colIndex + 1)) {
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

  navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem | null {
    let tries: Navigable[]

    switch (from) {
      case NavigationComingFrom.Above:
      case NavigationComingFrom.Below:
        const prio = this.getPriorityFocusItem()

        if (prio) {
          return prio
        }

        tries = this.columns
        break

      case NavigationComingFrom.Left:
        tries = this.columns
        break

      case NavigationComingFrom.Right:
        tries = [...this.columns].reverse()
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

  children(): Navigable[] {
    return this.columns
  }
}
