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
    switch (action) {
      case NavigationAction.Press:
        this.props.onPress?.()
        break

      case NavigationAction.LongPress:
        this.props.onLongPress?.()
        break

      case NavigationAction.Back:
        this.props.onBack?.()
        break
    }

    return null
  }

  canHandleDirection(_: NavigationDirection): boolean {
    return false
  }

  handleDirection(_: NavigationDirection): NavigableItem | null {
    throw new Error('Tried to make a simple navigable item handle a direction')
  }

  onFocus(): void {
    this.props.onFocusChange?.(true)
  }

  onUnfocus(): void {
    this.props.onFocusChange?.(false)
  }
}

export type SimpleNavigableItemProps = {
  onPress?: () => void
  onLongPress?: () => void
  onFocusChange?: (hasFocus: boolean) => void
  onBack?: () => void
  getUnderlyingElement: () => HTMLNavigableItemWrapperElement
}