import { useId } from 'react'
import {
  NAVIGABLE_DATA_ID_ATTR,
  NAVIGABLE_DATA_TYPE_ATTR,
  type NavigationManager,
  type RegistryItemNavType,
  type RegistryItemProps,
  type UntypedNavigablesSet,
} from '#/navigable/index.ts'
import { useOnMounted, useOnUnmounted, useValueIdentityWatcher } from '#/utils/hooks.ts'

export function useNavId(): string {
  const id = useId()
  return id
}

export function useNavigable<R extends UntypedNavigablesSet, N extends RegistryItemNavType<R>>(
  navManager: NavigationManager<R>,
  name: N,
  props: RegistryItemProps<R, N>,
  fixedNavId?: string,
): {
  navId: string
  domProps: Record<`data-${string}`, string>
  // updateProps: (newProps: RegistryItemProps<R, N>) => void
} {
  const generatedId = useId()
  const navId = fixedNavId ?? generatedId

  navManager.createNav(name, props, navId)

  useOnMounted(() => navManager.createNav(name, props, navId))
  useOnUnmounted(() => navManager.unregisterNav(navId))

  // TODO: this is triggered frequently, is is possible to fix?
  useValueIdentityWatcher(props, (newProps) => {
    navManager.updateNavProps(navId, name, newProps)
  })

  return {
    navId,
    domProps: { [NAVIGABLE_DATA_ID_ATTR]: navId, [NAVIGABLE_DATA_TYPE_ATTR]: name },
    // updateProps: (newProps) => {
    //   navManager.updateNavProps(navId, name, newProps)
    // },
  }
}
