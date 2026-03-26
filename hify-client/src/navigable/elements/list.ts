import type { NavigableHandler } from '..'

export type NavigableListProps = {
  focusChildIndexOnEnter?: number
  trapFocus?: boolean
}

export type NavigableListState = {
  position: { navId: string; index: number } | null
}

export const NavigableListHandler: NavigableHandler<
  'list',
  NavigableListProps,
  NavigableListState
> = {
  navType: 'list',
  isContainer: true,

  createState() {
    return { position: null }
  },

  enterFrom({ props, state, helpers }, fromDir) {
    const children = helpers.findChildren()

    if (props.focusChildIndexOnEnter !== undefined) {
      const index = props.focusChildIndexOnEnter

      if (index >= 0 && index < children.length) {
        return { type: 'focusChild', navId: children[index].id, fromDir }
      }

      console.warn(
        'Cannot focus child at index',
        index,
        'in NavigableList with',
        children.length,
        'children',
      )
    }

    if (children.length === 0) {
      return fromDir !== null ? { type: 'propagate' } : { type: 'cancel' }
    }

    switch (fromDir) {
      case 'NOWHERE':
      case 'LEFT':
      case 'RIGHT': {
        const { position } = state

        if (!position) {
          const [first] = children
          return { type: 'focusChild', navId: first.id, fromDir }
        }

        const child = children.find((child) => child.id === position.navId)

        if (child) {
          return { type: 'focusChild', navId: child.id, fromDir }
        }

        if (children.length > position.index) {
          return { type: 'focusChild', navId: children[position.index].id, fromDir }
        }

        const [first] = children
        return { type: 'focusChild', navId: first.id, fromDir }
      }

      case 'ABOVE': {
        const [first] = children
        return { type: 'focusChild', navId: first.id, fromDir }
      }

      case 'BELOW': {
        // oxlint-disable-next-line typescript/no-non-null-assertion
        return { type: 'focusChild', navId: children.at(-1)!.id, fromDir }
      }
    }
  },

  navigate(nav, focusedChild, dir) {
    const propagate =
      nav.props.trapFocus !== true
        ? ({ type: 'propagate' } as const)
        : ({ type: 'cancel' } as const)

    switch (dir) {
      case 'UP': {
        const children = nav.helpers.findChildren()
        const index = children.findIndex((s) => s.id === focusedChild.id)

        if (index > 0) {
          return {
            type: 'focusChild',
            navId: children[index - 1].id,
            fromDir: 'BELOW',
          }
        }

        return propagate
      }

      case 'DOWN': {
        const children = nav.helpers.findChildren()
        const index = children.findIndex((s) => s.id === focusedChild.id)

        if (index < children.length - 1) {
          return {
            type: 'focusChild',
            navId: children[index + 1].id,
            fromDir: 'ABOVE',
          }
        }

        return propagate
      }

      case 'LEFT':
      case 'RIGHT': {
        return propagate
      }
    }
  },

  onUnfocusedChild(nav, unfocusedChild, willUnfocusParent) {
    if (willUnfocusParent) {
      nav.state.position = {
        navId: unfocusedChild.id,
        index: nav.helpers.findChildren().findIndex((c) => c.id === unfocusedChild.id),
      }
    }
  },
}
