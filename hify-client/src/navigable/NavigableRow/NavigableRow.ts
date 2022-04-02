import { NavigableContainer, Navigable, NavigableItem, NavigationComingFrom, NavigationDirection } from '../navigation'

export class NavigableRow extends NavigableContainer {
  private readonly columns: Navigable[] = []

  append(navigable: Navigable) {
    this.columns.push(navigable)
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
          const item = colItem.firstItemDown(NavigationComingFrom.Right)

          if (item) {
            return item
          }
        }

        return this.parent.navigate(this, NavigationDirection.Right)

      case NavigationDirection.Right:
        for (const colItem of this.columns.slice(colIndex + 1)) {
          const item = colItem.firstItemDown(NavigationComingFrom.Left)

          if (item) {
            return item
          }
        }

        return this.parent.navigate(this, NavigationDirection.Left)

      case NavigationDirection.Down:
        return this.parent.navigate(this, NavigationDirection.Down)
    }
  }

  firstItemDown(from: NavigationComingFrom): NavigableItem | null {
    let tries: Navigable[]

    switch (from) {
      case NavigationComingFrom.Above:
      case NavigationComingFrom.Left:
      case NavigationComingFrom.Below:
        tries = this.columns
        break

      case NavigationComingFrom.Right:
        tries = [...this.columns].reverse()
        break
    }

    for (const child of tries) {
      const item = child.firstItemDown(from)

      if (item) {
        return item
      }
    }

    return null
  }
}
