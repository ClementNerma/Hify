import { NavigableContainer, Navigable, NavigableItem, NavigationComingFrom, NavigationDirection } from '../navigation'

export class NavigableGrid extends NavigableContainer {
  private readonly items: Navigable[] = []

  constructor(parent: NavigableContainer, private readonly columns: number) {
    super(parent)
  }

  private _rows() {
    return new Array(Math.ceil(this.items.length / this.columns))
      .fill(null)
      .map((_, i) => this.items.slice(i * this.columns, i * this.columns + this.columns))
  }

  append(navigable: Navigable) {
    this.items.push(navigable)
  }

  navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem | null {
    const itemIndex = this.items.indexOf(focusedChild)

    if (itemIndex === -1) {
      throw new Error('Focused element not found in navigable row')
    }

    const rows = this._rows()

    switch (direction) {
      case NavigationDirection.Up: {
        const rowIndex = Math.floor(itemIndex / this.columns)

        if (rowIndex === 0) {
          break
        }

        return rows[rowIndex - 1][itemIndex % this.columns].firstItemDown(NavigationComingFrom.Below)
      }

      case NavigationDirection.Down: {
        const rowIndex = Math.floor(itemIndex / this.columns)

        if (rowIndex === rows.length - 1) {
          break
        }

        return rows[rowIndex + 1][itemIndex % this.columns].firstItemDown(NavigationComingFrom.Above)
      }

      case NavigationDirection.Left:
      case NavigationDirection.Right: {
        const isLeft = direction === NavigationDirection.Left

        const row = rows.find((row) => row.indexOf(focusedChild) !== -1)

        if (!row) {
          throw new Error('Internal error: failed to find focused row in grid')
        }

        const sliced = isLeft
          ? row.slice(0, itemIndex % this.columns).reverse()
          : row.slice((itemIndex % this.columns) + 1)

        for (const colItem of sliced) {
          const item = colItem.firstItemDown(isLeft ? NavigationComingFrom.Right : NavigationComingFrom.Left)

          if (item) {
            return item
          }
        }

        break
      }
    }

    return this.parent.navigate(this, direction)
  }

  firstItemDown(from: NavigationComingFrom): NavigableItem | null {
    let rows = this._rows()

    switch (from) {
      case NavigationComingFrom.Above:
      case NavigationComingFrom.Left:
        break

      case NavigationComingFrom.Right:
        rows = rows.map((row) => [...row].reverse())
        break

      case NavigationComingFrom.Below:
        rows = [...rows].reverse()
        break
    }

    for (const row of rows) {
      for (const col of row) {
        const item = col.firstItemDown(from)

        if (item) {
          return item
        }
      }
    }

    return null
  }
}
