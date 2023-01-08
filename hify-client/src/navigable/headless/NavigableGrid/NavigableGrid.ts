import {
  Navigable,
  NavigableItem,
  NavigationComingFrom,
  NavigationDirection,
  NavigableContainer,
  NoProp,
} from '../../navigation'

export class NavigableGrid<P = NoProp> extends NavigableContainer<NavigableGridProps & P> {
  private itemsBeforeLastLazyLoading = 0

  private _makeRows(items: Navigable[]): Navigable[][] {
    return new Array(Math.ceil(items.length / this.props.columns))
      .fill(null)
      .map((_, i) => items.slice(i * this.props.columns, i * this.props.columns + this.props.columns))
  }

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

    const itemIndex = items.indexOf(focusedChild)

    if (itemIndex === -1) {
      throw new Error('Focused element not found in navigable row')
    }

    const rows = this._makeRows(items)

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
        if (rowIndex >= rows.length - 1 - (this._props.distanceBeforeLazyLoading ?? 3)) {
          this._lazyLoading(items)
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

  override navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem<unknown> | null {
    const items = this.children()

    let rows = this._makeRows(items)

    switch (from) {
      case NavigationComingFrom.Above:
      case NavigationComingFrom.Left:
        break

      case NavigationComingFrom.Right:
        rows = rows.map((row) => [...row].reverse())
        break

      case NavigationComingFrom.Below:
        rows = rows.reverse()
        this._lazyLoading(items)
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

  override navigateToLastItem(): NavigableItem<unknown> | null {
    this._lazyLoading(this.children())

    return super.navigateToLastItem()
  }
}

type NavigableGridProps = {
  columns: number
  lazyLoader?: () => void
  distanceBeforeLazyLoading?: number
}
