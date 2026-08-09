import type { NavigableHandler, NavigableOf, UntypedNavigable } from '..'

export type NavigableGridProps = {
  columns: number
  fetchMore?: {
    rowsEagerness: number
    debouncedLoader: () => void
  }
}

export type NavigableGridState = {
  position: {
    row: number
    col: number
    navId: string
  } | null
}

export const NavigableGridHandler: NavigableHandler<
  'grid',
  NavigableGridProps,
  NavigableGridState
> = {
  navType: 'grid',
  isContainer: true,

  createState() {
    return { position: null }
  },

  enterFrom(nav, fromDir) {
    const children = makeRows(nav)

    if (children.length === 0) {
      return { type: 'propagate' }
    }

    switch (fromDir) {
      case 'NOWHERE': {
        // TODO: restore state
        return { type: 'focusChild', navId: children[0][0].id, fromDir: 'NOWHERE' }
      }

      case 'ABOVE': {
        // TODO: use state for column
        return { type: 'focusChild', navId: children[0][0].id, fromDir: 'ABOVE' }
      }

      case 'LEFT': {
        // TODO: use state for row
        return { type: 'focusChild', navId: children[0][0].id, fromDir: 'LEFT' }
      }

      case 'RIGHT': {
        // TODO: use state for row
        return { type: 'focusChild', navId: children[0][children.length - 1].id, fromDir: 'RIGHT' }
      }

      case 'BELOW': {
        // TODO: use state for column
        return { type: 'focusChild', navId: children[children.length - 1][0].id, fromDir: 'BELOW' }
      }
    }
  },

  navigate(nav, focused, dir) {
    const rows = makeRows(nav)

    if (rows.length === 0) {
      return { type: 'propagate' }
    }

    const childIndex = rows.flat().findIndex((c) => c.id === focused.id)

    const childRow = Math.floor(childIndex / nav.props.columns)
    const childCol = childIndex % nav.props.columns

    switch (dir) {
      case 'UP': {
        return childRow > 0
          ? { type: 'focusChild', navId: rows[childRow - 1][childCol].id, fromDir: 'BELOW' }
          : { type: 'propagate' }
      }

      case 'DOWN': {
        if (nav.props.fetchMore && childRow >= rows.length - nav.props.columns) {
          nav.props.fetchMore.debouncedLoader()
        }

        if (childRow < rows.length - 2) {
          return { type: 'focusChild', navId: rows[childRow + 1][childCol].id, fromDir: 'ABOVE' }
        }

        if (childRow === rows.length - 2) {
          const lastRow = rows[rows.length - 1]

          if (childCol < lastRow.length) {
            return { type: 'focusChild', navId: lastRow[childCol].id, fromDir: 'ABOVE' }
          }
        }

        return { type: 'propagate' }
      }

      case 'LEFT': {
        return childCol > 0
          ? { type: 'focusChild', navId: rows[childRow][childCol - 1].id, fromDir: 'RIGHT' }
          : { type: 'propagate' }
      }

      case 'RIGHT': {
        return childCol < nav.props.columns - 1 && childCol < rows[childRow].length - 1
          ? { type: 'focusChild', navId: rows[childRow][childCol + 1].id, fromDir: 'LEFT' }
          : { type: 'propagate' }
      }
    }
  },
}

function makeRows(grid: NavigableOf<typeof NavigableGridHandler>): UntypedNavigable[][] {
  const children = grid.helpers.findChildren()

  return Array.from({ length: Math.ceil(children.length / grid.props.columns) }).map((_, i) =>
    children.slice(i * grid.props.columns, i * grid.props.columns + grid.props.columns),
  )
}
