import {
  HTMLNavigableItemWrapperElement,
  NavigableContainer,
  NavigableItem,
  NavigationAction,
  NavigationDirection,
} from '../navigation'

export class SimpleNavigableItem extends NavigableItem {
  constructor(parent: NavigableContainer, private readonly props: SimpleNavigableItemProps) {
    super(parent)

    if (props.hasFocusPriority !== undefined && props.focuser === undefined) {
      throw new Error('Cannot provide a focus priority indicator without a focuser!')
    }
  }

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
    }
  }

  handleAction(action: NavigationAction): NavigableItem | null {
    const callbacks: { [action in NavigationAction]: SimpleNavigableItemCallback | undefined } = {
      [NavigationAction.Press]: this.props.onPress,
      [NavigationAction.LongPress]: this.props.onLongPress,
      [NavigationAction.Back]: this.props.onBack,
    }

    const fn = callbacks[action]

    if (!fn) {
      throw new Error('Tried to call unsupported action callback on navigable item')
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

  handleDirection(direction: NavigationDirection): NavigableItem | null {
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
    this.props.onFocusChange?.(true)
  }

  onUnfocus(): void {
    this.props.onFocusChange?.(false)
  }

  hasFocusPriority(): boolean | null {
    return this.props.hasFocusPriority ?? null
  }
}

export type SimpleNavigableItemCallback = () => NavigableItem | null | void

export type SimpleNavigableItemProps = {
  onFocusChange?: (hasFocus: boolean) => void

  onPress?: SimpleNavigableItemCallback
  onLongPress?: SimpleNavigableItemCallback
  onBack?: SimpleNavigableItemCallback

  onUp?: SimpleNavigableItemCallback
  onLeft?: SimpleNavigableItemCallback
  onRight?: SimpleNavigableItemCallback
  onDown?: SimpleNavigableItemCallback

  getUnderlyingElement: () => HTMLNavigableItemWrapperElement

  focuser?: NavigableContainer
  hasFocusPriority?: boolean
}
