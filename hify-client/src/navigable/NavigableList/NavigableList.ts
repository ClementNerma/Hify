import { NavigableContainer, Navigable, NavigableItem, NavigationComingFrom, NavigationDirection } from '../navigation'

export class NavigableList extends NavigableContainer {
  private readonly rows: Navigable[] = []

  append(navigable: Navigable) {
    this.rows.push(navigable)
  }

  remove(child: Navigable): void {
    const index = this.rows.indexOf(child)

    if (index === -1) {
      throw new Error('Tried to remove unknown child from navigable list')
    }

    this.rows.splice(index, 1)
  }

  hasChild(child: Navigable): boolean {
    return this.rows.indexOf(child) !== -1
  }

  navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem | null {
    const rowIndex = this.rows.indexOf(focusedChild)

    if (rowIndex === -1) {
      throw new Error('Focused element not found in navigable list')
    }

    switch (direction) {
      case NavigationDirection.Up:
        for (const rowItem of [...this.rows.slice(0, rowIndex)].reverse()) {
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
        for (const rowItem of this.rows.slice(rowIndex + 1)) {
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
      case NavigationComingFrom.Above:
      case NavigationComingFrom.Left:
      case NavigationComingFrom.Right:
        tries = this.rows
        break

      case NavigationComingFrom.Below:
        tries = [...this.rows].reverse()
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

  lastItem(): NavigableItem | null {
    return this.rows.at(-1)?.lastItem() ?? null
  }
}
