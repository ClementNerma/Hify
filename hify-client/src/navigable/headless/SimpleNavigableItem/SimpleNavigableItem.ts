import {
  HTMLNavigableItemWrapperElement,
  NavigableItem,
  NavigationAction,
  NavigationDirection,
  NoProp,
} from '../../navigation'

export class SimpleNavigableItem<P = NoProp> extends NavigableItem<P & SimpleNavigableItemProps> {
  underlyingElement(): HTMLNavigableItemWrapperElement {
    return this.props.getUnderlyingElement()
  }

  canHandleAction(key: NavigationAction): boolean {
    switch (key) {
      case NavigationAction.Press:
        return !!this.props.onPress

      case NavigationAction.LongPress:
        return !!this.props.onLongPress

      case NavigationAction.Back:
        return !!this.props.onBack

      case NavigationAction.LongBack:
        return false
    }
  }

  handleAction(action: NavigationAction): NavigableItem<unknown> | null {
    const callbacks: { [action in NavigationAction]: SimpleNavigableItemCallback | undefined } = {
      [NavigationAction.Press]: this.props.onPress,
      [NavigationAction.LongPress]: this.props.onLongPress,
      [NavigationAction.Back]: this.props.onBack,
      [NavigationAction.LongBack]: () => null,
    }

    const fn = callbacks[action]

    if (!fn) {
      throw new Error('Tried to call unsupported action callback on navigable item')
    }

    if (this.props.disabled === true) {
      return null
    }

    return fn() ?? null
  }

  canHandleDirection(direction: NavigationDirection): boolean {
    switch (direction) {
      case NavigationDirection.Up:
        return !!this.props.onUp

      case NavigationDirection.Left:
        return !!this.props.onLeft

      case NavigationDirection.Right:
        return !!this.props.onRight

      case NavigationDirection.Down:
        return !!this.props.onDown
    }
  }

  handleDirection(direction: NavigationDirection): NavigableItem<unknown> | null {
    const callbacks: { [action in NavigationDirection]: SimpleNavigableItemCallback | undefined } = {
      [NavigationDirection.Up]: this.props.onUp,
      [NavigationDirection.Left]: this.props.onLeft,
      [NavigationDirection.Right]: this.props.onRight,
      [NavigationDirection.Down]: this.props.onDown,
    }

    const fn = callbacks[direction]

    if (!fn) {
      throw new Error('Tried to call unsupported direction callback on navigable item')
    }

    return fn() ?? null
  }

  onFocus(): void {
    this.props.onFocus?.()
  }

  onUnfocus(): void {
    this.props.onUnfocus?.()
  }

  // Additional methods
  requestFocus(): boolean {
    this.parent.page.requestPageFocus(this)
    return true
  }
}

export type SimpleNavigableItemCallback = () => NavigableItem<unknown> | null | void

export type SimpleNavigableItemProps = {
  disabled?: boolean

  onFocus?: () => void
  onUnfocus?: () => void

  onPress?: SimpleNavigableItemCallback
  onLongPress?: SimpleNavigableItemCallback
  onBack?: SimpleNavigableItemCallback

  onUp?: SimpleNavigableItemCallback
  onLeft?: SimpleNavigableItemCallback
  onRight?: SimpleNavigableItemCallback
  onDown?: SimpleNavigableItemCallback

  getUnderlyingElement: () => HTMLNavigableItemWrapperElement
}
