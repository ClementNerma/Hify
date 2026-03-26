import { NavItem } from '#/components/navigables/Item.tsx'
import { NavRow } from '#/components/navigables/Row.tsx'
import { navigationManager } from '#/global/nav.ts'
import { showFailure } from '#/global/notifications.ts'
import { navigate, type RouteParams } from '#/router/routes.ts'
import { routes, type Routes } from '#/routes.ts'
import { useValueWatcher } from '#/utils/hooks.ts'

export type NavBarProps = {
  tabs: NavBarTab[]
  currentRouteName: keyof Routes | null
}

export type NavBarTab = {
  label: string
  route: NavBarRoute
  childrenRoutes?: (keyof Routes)[]
}

export type NavBarRoute = {
  name: keyof Routes
  props: RouteParams<Routes[keyof Routes]>
}

export function NavBar({ tabs, currentRouteName }: NavBarProps) {
  useValueWatcher(
    currentRouteName,
    (currentRouteName) => {
      // Better than a fallback to 'home', as it to avoid focusing on the 'home' tab briefly before the actual tab
      // when starting navigation on a specific page (where `currentRouteName` would be `null` until the page is ready).
      if (!currentRouteName) {
        return
      }

      // TODO: check tabs coverage ahead-of-time, and build a map for O(1) access here
      const tab = tabs.find(
        (tab) =>
          tab.route.name === currentRouteName ||
          (tab.childrenRoutes?.includes(currentRouteName) ?? false),
      )

      if (!tab) {
        showFailure(`No tab found matching route ${currentRouteName}`)
        return
      }

      navigationManager.focusById(`nav-route-${tab.route.name}`, null)
    },
    { immediate: true },
  )

  return (
    <NavRow>
      {tabs.map(({ label, route: { name, props } }) => (
        <NavItem
          key={name}
          onPress={() => navigate(routes[name], props)}
          fixedNavId={`nav-route-${name}`}
          onFocused={scrollToTop}
        >
          <span className="p-2">{label}</span>
        </NavItem>
      ))}
    </NavRow>
  )
}

// When scrolling down the page and then coming back to the nav bar, the scroll will be slightly off
// as the navigation manager only scrolls enough to make the focused element visible.
//
// To avoid this problem, we scroll to the top of the page when focusing a nav item.
function scrollToTop() {
  window.scrollTo({ top: 0 })
}
