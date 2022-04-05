import {
  Navigable,
  NavigableContainer,
  NavigableItem,
  NavigationAction,
  NavigationComingFrom,
  NavigationDirection,
} from '../navigation'

export type NavigableWithHandlersProps = {
  onPress?: () => NavigableItem | null | void
  onLongPress?: () => NavigableItem | null | void
  onBack?: () => NavigableItem | null | void
}

export class NavigableWithHandlers extends NavigableContainer {
  private onlyChild: Navigable | null = null

  constructor(parent: NavigableContainer, private readonly props: NavigableWithHandlersProps) {
    super(parent)
  }

  append(navigable: Navigable): void {
    if (this.onlyChild) {
      throw new Error('Navigable with handlers can only contain a single navigable')
    }

    this.onlyChild = navigable
  }

  hasChild(child: Navigable): boolean {
    return child === this.onlyChild
  }

  remove(child: Navigable): void {
    if (!this.onlyChild) {
      throw new Error('Cannot remove component from empty navigable with handlers')
    }

    if (this.onlyChild !== child) {
      throw new Error("Tried to remove another navigable than the navigable's only one")
    }

    this.onlyChild = null
  }

  navigate(_: NavigableContainer, direction: NavigationDirection): NavigableItem | null {
    return this.parent.navigate(this, direction)
  }

  navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem | null {
    return this.onlyChild ? this.onlyChild.navigateToFirstItemDown(from) : null
  }

  navigateToLastItem(): NavigableItem | null {
    return this.onlyChild?.navigateToLastItem() ?? null
  }

  override canHandleAction(action: NavigationAction): boolean {
    switch (action) {
      case NavigationAction.Press:
        return !!this.props.onPress

      case NavigationAction.LongPress:
        return !!this.props.onLongPress

      case NavigationAction.Back:
        return !!this.props.onBack
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
    }
  }
}
