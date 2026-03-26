import { useRef, useState, type ReactNode } from 'react'
import { isDfModeFeatureEnabledStore } from '#/global/df-mode.ts'
import { navigationManager } from '#/global/nav.ts'
import { showFailure } from '#/global/notifications.ts'
import { assertNotNull } from '#/utils/common.ts'
import { useNavId } from '../navigables/hooks'
import { BlockNavItem } from '../navigables/Item'
import { NavList } from '../navigables/List'

export type ContextMenuProps = {
  status: ContextMenuStatus
  onClose: () => void
}

export type ContextMenuStatus =
  | { type: 'opened'; items: (ContextMenuItem & { id: string })[] }
  | { type: 'closed' }

export type ContextMenuItem = {
  icon: ReactNode
  label: string
  onPress: () => void
  skipFocusRestore?: boolean
}

// TODO: close when clicking or right-clicking outside
export function ContextMenu({ status, onClose }: ContextMenuProps) {
  const domRef = useRef<HTMLDivElement>(null)
  const opened = useRef(false)

  const [openedWith, setOpenedWith] = useState<{
    top: number
    left: number
    prevNavId: string
  } | null>(null)

  const navId = useNavId()

  const onReady = (firstItemNavId: string) => {
    if (opened.current) {
      return
    }

    opened.current = true

    isDfModeFeatureEnabledStore.mutate(false)

    assertNotNull(domRef.current)

    const focusedId = navigationManager.focusedId()

    if (focusedId === null) {
      showFailure('No focused nav when opening context menu')
      return
    }

    const focusedDom = navigationManager.findDomById(focusedId)

    setOpenedWith({
      ...computeContextMenuPosition(focusedDom, domRef.current),
      prevNavId: focusedId,
    })

    navigationManager.focusById(firstItemNavId, null)
  }

  const closeMenu = (skipFocusRestore?: boolean) => {
    assertNotNull(openedWith)
    onClose()
    setOpenedWith(null)
    opened.current = false

    isDfModeFeatureEnabledStore.mutate(true)

    if (skipFocusRestore !== true) {
      const { prevNavId } = openedWith

      if (prevNavId) {
        navigationManager.focusById(prevNavId, null)
      }
    }
  }

  const onItemPress = (item: ContextMenuItem) => {
    closeMenu(item.skipFocusRestore)
    item.onPress()
  }

  if (status.type === 'closed') {
    return null
  }

  if (status.items.length === 0) {
    showFailure('Context menu opened with no items')
    return null
  }

  return (
    <div
      className={`fixed max-w-80 bg-gray-800 border border-gray-700 shadow-lg z-50 ${openedWith ? '' : 'opacity-0'}`}
      style={openedWith ? { top: openedWith.top, left: openedWith.left } : {}}
      ref={domRef}
    >
      <NavList
        className="grid grid-cols-[max-content_1fr_max-content]"
        fixedNavId={navId}
        onBackKey={closeMenu}
        trapFocus
      >
        {status.items.map((item, i) => {
          const { id, icon, label } = item

          return (
            <BlockNavItem
              key={id}
              className="grid grid-cols-subgrid col-span-full p-2 not-last:border-b-gray-700 nav-focused:bg-gray-700"
              onPress={() => onItemPress(item)}
              onReady={i === 0 ? onReady : null}
            >
              <span className="mr-4">{icon}</span>
              <span className="line-clamp-1">{label}</span>
            </BlockNavItem>
          )
        })}
      </NavList>
    </div>
  )
}

function computeContextMenuPosition(
  from: Element,
  ctxMenuContainer: Element,
): { top: number; left: number } {
  const fromRect = from.getBoundingClientRect()

  const top = fromRect.top + fromRect.height / 2
  const left = fromRect.left + Math.min(30, fromRect.width / 2)

  return {
    top:
      top + ctxMenuContainer.clientHeight > window.innerHeight
        ? window.innerHeight - ctxMenuContainer.clientHeight - 5
        : top,

    left:
      left + ctxMenuContainer.clientWidth > window.innerWidth
        ? window.innerWidth - ctxMenuContainer.clientWidth - 5
        : left,
  }
}
