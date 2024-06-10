import type { InputHandler, KeyPressHandling } from '../../input-manager'
import {
	NavigableContainer,
	NavigationAction,
	type Navigable,
	type NavigableItem,
	type NavigationDirection,
	type NoProp,
} from '../../navigation'

export class NavigableWithHandlers<P = NoProp> extends NavigableContainer<NavigableWithHandlersProps & P> {
	navigate(_: Navigable, direction: NavigationDirection): NavigableItem<unknown> | null {
		return this.parent.navigate(this, direction)
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

	override handleAction(action: NavigationAction): null {
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

			case NavigationAction.LongBack:
				this.props.onLongBack?.()
				break
		}

		return null
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
