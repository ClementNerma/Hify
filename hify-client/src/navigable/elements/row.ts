import type { NavigableHandler } from '..'

export type NavigableRowProps = {
  focusChildOnEnter?: number | null
}

export type NavigableRowState = {
  position: { index: number; navId: string } | null
}

export const NavigableRowHandler: NavigableHandler<'row', NavigableRowProps, NavigableRowState> = {
  navType: 'row',
  isContainer: true,

  createState() {
    return { position: null }
  },

  enterFrom({ state, helpers, props }, fromDir) {
    const children = helpers.findChildren()

    if (children.length === 0) {
      return fromDir !== null ? { type: 'propagate' } : { type: 'cancel' }
    }

    if (props.focusChildOnEnter !== undefined) {
      const index = props.focusChildOnEnter

      if (index !== null && index >= 0 && index < children.length) {
        return { type: 'focusChild', navId: children[index].id, fromDir }
      }
    }

    switch (fromDir) {
      case 'NOWHERE':
      case 'ABOVE':
      case 'BELOW': {
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

      case 'LEFT': {
        const [first] = children

        return { type: 'focusChild', navId: first.id, fromDir }
      }

      case 'RIGHT': {
        // oxlint-disable-next-line typescript/no-non-null-assertion
        return { type: 'focusChild', navId: children.at(-1)!.id, fromDir }
      }
    }
  },

  navigate(nav, focusedChild, dir) {
    switch (dir) {
      case 'LEFT': {
        const children = nav.helpers.findChildren()
        const index = children.findIndex((s) => s.id === focusedChild.id)

        if (index > 0) {
          return {
            type: 'focusChild',
            navId: children[index - 1].id,
            fromDir: 'RIGHT',
          }
        }

        return { type: 'propagate' }
      }

      case 'RIGHT': {
        const children = nav.helpers.findChildren()
        const index = children.findIndex((s) => s.id === focusedChild.id)

        if (index < children.length - 1) {
          return {
            type: 'focusChild',
            navId: children[index + 1].id,
            fromDir: 'LEFT',
          }
        }

        return { type: 'propagate' }
      }

      case 'UP':
      case 'DOWN': {
        return { type: 'propagate' }
      }
    }
  },

  onUnfocusedChild(nav, unfocusedChild, willUnfocusParent) {
    if (willUnfocusParent) {
      const index = nav.helpers.findChildren().findIndex((c) => c.id === unfocusedChild.id)
      nav.state.position = { index, navId: unfocusedChild.id }
    }
  },
}
