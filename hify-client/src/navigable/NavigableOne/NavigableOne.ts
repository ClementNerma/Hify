import { Navigable, NavigableContainer, NavigableItem, NavigationComingFrom, NavigationDirection } from '../navigation'

export class NavigableOne<P = {}> extends NavigableContainer<P> {
  private onlyChild: Navigable | null = null
  readonly ordered = false

  append(navigable: Navigable): void {
    if (this.onlyChild) {
      throw new Error('Navigable ones can only contain a single navigable')
    }

    this.onlyChild = navigable
  }

  hasChild(child: Navigable): boolean {
    return child === this.onlyChild
  }

  remove(child: Navigable): void {
    if (!this.onlyChild) {
      throw new Error('Cannot remove component from empty navigable one')
    }

    if (this.onlyChild !== child) {
      throw new Error("Tried to remove another navigable than the navigable's only one")
    }

    this.onlyChild = null
  }

  navigate(focusedChild: NavigableContainer, direction: NavigationDirection): NavigableItem<unknown> | null {
    if (focusedChild !== this.onlyChild) {
      throw new Error('Focused child does not belong to the current single container')
    }

    return this.parent.navigate(this, direction)
  }

  navigateToFirstItemDown(from: NavigationComingFrom): NavigableItem<unknown> | null {
    return this.onlyChild ? this.onlyChild.navigateToFirstItemDown(from) : null
  }

  navigateToLastItem(): NavigableItem<unknown> | null {
    return this.onlyChild?.navigateToLastItem() ?? null
  }

  requestFocus(): boolean {
    return this.onlyChild?.requestFocus() ?? false
  }
}
