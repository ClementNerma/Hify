import { NavigableOne } from '../NavigableOne/NavigableOne'
import { NavigableContainer, NavigableItem, NavigationAction } from '../navigation'

export type NavigableWithHandlersProps = {
  position: number | null
  hasFocusPriority: boolean | null

  onPress?: () => NavigableItem | null | void
  onLongPress?: () => NavigableItem | null | void
  onBack?: () => NavigableItem | null | void
  onLongBack?: () => NavigableItem | null | void

  onKeyPress?: (key: string) => boolean | void
}

export class NavigableWithHandlers extends NavigableOne {
  constructor(parent: NavigableContainer, public props: NavigableWithHandlersProps) {
    super(parent, props.position, props.hasFocusPriority)
  }

  override canHandleAction(action: NavigationAction): boolean {
    switch (action) {
      case NavigationAction.Press:
        return !!this.props.onPress

      case NavigationAction.LongPress:
        return !!this.props.onLongPress

      case NavigationAction.Back:
        return !!this.props.onBack

      case NavigationAction.LongBack:
        return !!this.props.onLongBack
    }
  }

  override handleAction(action: NavigationAction): NavigableItem | null {
    switch (action) {
      case NavigationAction.Press:
        return this.props.onPress?.() ?? null

      case NavigationAction.LongPress:
        return this.props.onLongPress?.() ?? null

      case NavigationAction.Back:
        return this.props.onBack?.() ?? null

      case NavigationAction.LongBack:
        return this.props.onLongBack?.() ?? null
    }
  }

  override interceptKey(key: string): boolean | void {
    return this.props.onKeyPress?.(key)
  }
}
