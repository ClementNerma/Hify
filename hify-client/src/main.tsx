import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
// oxlint-disable-next-line no-unassigned-import
import './main.css'
import { App } from './App.tsx'
import { loadPersistedPlayerState } from './global/player.ts'
import { setupInputHandler } from './input.ts'

// oxlint-disable-next-line typescript/no-floating-promises
loadPersistedPlayerState()

// oxlint-disable-next-line no-non-null-assertion
createRoot(document.querySelector('#root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)

setupInputHandler()

// Prevent scroll restoration on back/forward navigation, as it can cause unwanted scroll jumps when navigating between views
if ('scrollRestoration' in history) {
  history.scrollRestoration = 'manual'
}
