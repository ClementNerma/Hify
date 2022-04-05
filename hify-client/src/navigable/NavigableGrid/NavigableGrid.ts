import {
  NavigableContainer,
  Navigable,
  NavigableItem,
  NavigationComingFrom,
  NavigationDirection,
  NavigableArrayContainer,
  NavigableAttachedData,
} from '../navigation'

export class NavigableGrid extends NavigableContainer {
  private readonly items: Navigable[] = []
  private itemsBeforeLastLazyLoading = 0

  constructor(parent: NavigableContainer, private readonly props: NavigableGridProps) {
    super(parent, props.attachedData)
  }

  private _rows() {
    return new Array(Math.ceil(this.items.length / this.props.columns))
      .fill(null)
      .map((_, i) => this.items.slice(i * this.props.columns, i * this.props.columns + this.props.columns))
  }

  private _lazyLoading() {
    if (!this.props.lazyLoader) {
      return
    }

    if (this.itemsBeforeLastLazyLoading !== this.items.length) {
      this.itemsBeforeLastLazyLoading = this.items.length
      this.props.lazyLoader()
    }
  }

  append(navigable: Navigable) {
    this.items.push(navigable)
  }

  hasChild(child: Navigable): boolean {
    return this.items.indexOf(child) !== -1
  }

  remove(child: Navigable): void {
    const index = this.items.indexOf(child)

    if (index === -1) {
      throw new Error('Tried to remove unknown child from navigable grid')
    }

    this.items.splice(index, 1)
  }

  navigate(focusedChild: Navigable, direction: NavigationDirection): NavigableItem | null {
    const itemIndex = this.items.indexOf(focusedChild)

    if (itemIndex === -1) {
      throw new Error('Focused element not found in navigable row')
    }

    const rows = this._rows()

    switch (direction) {
      case NavigationDirection.Up: {
        const rowIndex = Math.floor(itemIndex / this.props.columns)

        if (rowIndex === 0) {
          break
        }

        return rows[rowIndex - 1][itemIndex % this.props.columns].navigateToFirstItemDown(NavigationComingFrom.Below)
      }

      case NavigationDirection.Down: {
        const rowIndex = Math.floor(itemIndex / this.props.columns)

        // Required to trigger lazy loader when either:
        // * We navigate to the last row from the above one
        // * We navigate to the last row from below
        if (rowIndex >= rows.length - 2) {
          this._lazyLoading()
        }

        if (rowIndex === rows.length - 1) {
          break
        }

        const newRow = rows[rowIndex + 1]
        const newItemIndex = Math.min(itemIndex % this.props.columns, newRow.length - 1)

        return newRow[newItemIndex].navigateToFirstItemDown(NavigationComingFrom.Above)
      }

      case NavigationDirection.Left:
      case NavigationDirection.Right: {
        const isLeft = direction === NavigationDirection.Left

        const row = rows.find((row) => row.indexOf(focusedChild) !== -1)

        if (!row) {
          throw new Error('Internal error: failed to find focused row in grid')
        }

        const sliced = isLeft
          ? row.slice(0, itemIndex % this.props.columns).reverse()
          : row.slice((itemIndex % this.props.columns) + 1)

        for (const colItem of sliced) {
          const item = colItem.navigateToFirstItemDown(isLeft ? NavigationComingFrom.Right : NavigationComingFrom.Left)

          if (item) {
            return item
          }
        }

        break
      }
    }

    return this.parent.navigate(this, direction)
  }

  navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem | null {
    let rows = this._rows()

    switch (from) {
      case NavigationComingFrom.Above:
      case NavigationComingFrom.Left:
        break

      case NavigationComingFrom.Right:
        rows = rows.map((row) => [...row].reverse())
        break

      case NavigationComingFrom.Below:
        rows = rows.reverse()
        this._lazyLoading()
        break
    }

    for (const row of rows) {
      for (const col of row) {
        const item = col.navigateToFirstItemDown(from)

        if (item) {
          return item
        }
      }
    }

    return null
  }

  navigateToLastItem(): NavigableItem | null {
    this._lazyLoading()
    return NavigableArrayContainer.navigateToLastItem(this.items)
  }
}

export type NavigableGridProps = {
  columns: number
  lazyLoader?: () => void
  attachedData: NavigableAttachedData
}
