import React, { useState, type LazyExoticComponent } from 'react'
import { IconContext } from 'react-icons'
import { clearQueriesCache } from './api/hooks'
import { ErrorBoundary } from './components/organisms/ErrorBoundary'
import { setInputFrozen } from './input'
import { Layout } from './Layout'
import { type RouteRenderers, Router } from './router/Router'
import { routes, type Routes } from './routes'
import { useValueWatcher } from './utils/hooks'

export function App() {
  const [isLoading, setIsLoading] = useState(false)
  const [currentRouteName, setCurrentRouteName] = useState<keyof Routes | null>(null)

  // Prevent user inputs while loading a new page
  // (as the focus will jump on the nav bar when the new page is ready)
  useValueWatcher(isLoading, setInputFrozen, { immediate: true })

  return (
    <IconContext value={{ style: { display: 'inline-block' } }}>
      <div className="fixed inset-0 -z-30 bg-[linear-gradient(to_bottom,#363636_0vh,#080808_33vh)]" />

      <div className="min-h-screen min-w-screen font-mono p-2 leading-6 text-[#d7dadc] text-[15px] not-lg:text-[12px] selection:bg-indigo-500">
        <ErrorBoundary>
          <Layout isLoading={isLoading} currentRouteName={currentRouteName}>
            <Router
              routes={routes}
              renderers={routeRenderers}
              fallback={() => <h1>404</h1>}
              onLoading={() => {
                clearQueriesCache()
                setIsLoading(true)
              }}
              onPageReady={(routeName) => {
                setIsLoading(false)
                setCurrentRouteName(routeName)
              }}
            />
          </Layout>
        </ErrorBoundary>
      </div>
    </IconContext>
  )
}

const routeRenderers: RouteRenderers<Routes> = {
  home: lazy(async () => import('./views/HomeView'), 'HomeView'),

  albums: lazy(async () => import('./views/AlbumsView'), 'AlbumsView'),
  album: lazy(async () => import('./views/AlbumView'), 'AlbumView'),

  artists: lazy(async () => import('./views/ArtistsView'), 'ArtistsView'),
  artist: lazy(async () => import('./views/ArtistView'), 'ArtistView'),

  genres: lazy(async () => import('./views/GenresView'), 'GenresView'),
  genre: lazy(async () => import('./views/GenreView'), 'GenreView'),

  search: lazy(async () => import('./views/SearchView'), 'SearchView'),

  player: lazy(async () => import('./views/PlayerView'), 'PlayerView'),
  history: lazy(async () => import('./views/HistoryView'), 'HistoryView'),
  tools: lazy(async () => import('./views/ToolsView'), 'ToolsView'),
}

function lazy<T extends { [_ in K]: React.FC<P> }, K extends keyof T, P>(
  load: () => Promise<T>,
  key: K,
): LazyExoticComponent<React.FC<P>> {
  return React.lazy(async () => {
    const mod = await load()
    return { default: mod[key] }
  })
}
