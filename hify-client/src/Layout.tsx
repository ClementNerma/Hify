import { useState, type PropsWithChildren } from 'react'
import { LoadingIndicator } from './components/atoms/LoadingIndicator'
import { DfFadeOut } from './components/molecules/DfToggle'
import { FadeWhenUnfocused } from './components/molecules/FadeIfUnfocused'
import { NavList } from './components/navigables/List'
import { ContextMenu } from './components/organisms/ContextMenu'
import { NavBar, type NavBarTab } from './components/organisms/NavBar'
import { NotificationsContainer } from './components/organisms/Notifications'
import { closeContextMenu, ctxMenuStatusStore } from './global/ctx-menu'
import { navigationManager } from './global/nav'
import type { Routes } from './routes'
import { useValueWatcher } from './utils/hooks'
import { useGlobalStore } from './utils/stores'

export function Layout({
  children,
  isLoading,
  currentRouteName,
}: PropsWithChildren<{ isLoading: boolean; currentRouteName: keyof Routes | null }>) {
  const ctxMenuStatus = useGlobalStore(ctxMenuStatusStore)

  return (
    <NavList onBackKey={() => history.go(-1) /* TODO: improve */}>
      {/* Context menu container */}
      <ContextMenu status={ctxMenuStatus} onClose={closeContextMenu} />

      {/* Notifications container */}
      <NotificationsContainer />

      {/* Colored spinner shown during navigation */}
      <NavigationSpinner isLoading={isLoading} />

      {/* Navigation bar */}
      <DfFadeOut>
        <FadeWhenUnfocused>
          <div className="flex justify-center pb-2.5">
            <NavBar tabs={tabs} currentRouteName={currentRouteName} />
          </div>
        </FadeWhenUnfocused>
      </DfFadeOut>

      {/* Add some small spacing between the nav bar and the page's content */}
      <div className="h-4" />

      {/* Current page's view */}
      {children}
    </NavList>
  )
}

function NavigationSpinner({ isLoading }: { isLoading: boolean }) {
  const [loaderPos, setLoaderPos] = useState<{ top: number; left: number } | null>(null)

  useValueWatcher(
    isLoading,
    (isLoading) => {
      if (!isLoading) {
        setLoaderPos(null)
        return
      }

      const focusedNavId = navigationManager.focusedId()

      if (focusedNavId === null) {
        return
      }

      const focusedDom = navigationManager.findDomById(focusedNavId)

      const rect = focusedDom.getBoundingClientRect()

      setLoaderPos({
        top: rect.bottom > window.innerHeight - 15 ? rect.top : rect.bottom,
        left: rect.right > window.innerWidth - 15 ? rect.left : rect.right,
      })
    },
    { immediate: true },
  )

  return loaderPos && <LoadingIndicator size={12} top={loaderPos.top} left={loaderPos.left} />
}

const tabs: NavBarTab[] = [
  { label: 'Home', route: { name: 'home', props: {} } },
  { label: 'History', route: { name: 'history', props: {} } },
  { label: 'Player', route: { name: 'player', props: {} } },
  {
    label: 'Albums',
    route: { name: 'albums', props: {} },
    childrenRoutes: ['album'],
  },
  {
    label: 'Artists',
    route: { name: 'artists', props: {} },
    childrenRoutes: ['artist'],
  },
  {
    label: 'Genres',
    route: { name: 'genres', props: {} },
    childrenRoutes: ['genre'],
  },
  {
    label: 'Search',
    route: { name: 'search', props: {} },
  },
  {
    label: 'Tools',
    route: { name: 'tools', props: {} },
  },
]
