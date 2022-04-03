import { Navigable, NavigableContainer, NavigableItem, NavigationComingFrom, NavigationDirection } from '../navigation'

export class NavigableOne extends NavigableContainer {
  private onlyChild: Navigable | null = null

  constructor(parent: NavigableContainer) {
    super(parent)
  }

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

  navigate(_: NavigableContainer, direction: NavigationDirection): NavigableItem | null {
    return this.parent.navigate(this, direction)
  }

  firstItemDown(from: NavigationComingFrom): NavigableItem | null {
    return this.onlyChild ? this.onlyChild.firstItemDown(from) : null
  }

  lastItem(): NavigableItem | null {
    return this.onlyChild?.lastItem() ?? null
  }
}