import { InputHandler, KeyPressHandling } from '../../input-manager'
import { NavigableOne } from '../NavigableOne/NavigableOne'
import { NavigableItem, NavigationAction } from '../../navigation'

export class NavigableWithHandlers<P = {}> extends NavigableOne<NavigableWithHandlersProps & P> {
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

  override handleAction(action: NavigationAction): NavigableItem<unknown> | null {
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

  override interceptKeyPress(key: string, long: boolean): KeyPressHandling | void {
    return this.props.onKeyPress?.(key, long)
  }
}

export type NavigableWithHandlersProps = {
  onPress?: () => NavigableItem<unknown> | null | void
  onLongPress?: () => NavigableItem<unknown> | null | void
  onBack?: () => NavigableItem<unknown> | null | void
  onLongBack?: () => NavigableItem<unknown> | null | void

  onKeyPress?: InputHandler
}
