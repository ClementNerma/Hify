<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableList = NavigableCommonElementProps & { type: 'list' }

export type NavigableListProps = {
	interceptKeyPress?: (navigationKey: NavigationDirection | null, key: KeyPress, list: NavigableList) => boolean

	onFocus?: (list: NavigableList, focusedChild: NavigableElement) => void
	onUnfocus?: (list: NavigableList, unfocusedChild: NavigableElement) => void
	onNavigate?: (key: NavigationDirection, currentChild: NavigableElement, list: NavigableList) => void
	onEnter?: (from: NavigationDirection, list: NavigableList) => void
	onLeftKey?: (list: NavigableList) => void
	onRightKey?: (list: NavigableList) => void
	onUpKey?: (list: NavigableList) => void
	onDownKey?: (list: NavigableList) => void
	onBackKey?: (list: NavigableList) => void

	trapped?: boolean
} & Omit<NavigableList, 'id' | 'type'>

export type NavigableListExposeType = {
	list: NavigableList
	focused: boolean
}
</script>

<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref, } from 'vue';
import { NavigationDirection, generateNavigableElementId, navigableElementAttrs, registerNavigableElementHandlers, translateNavigationKey, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type KeyPress, type NavigableCommonElementProps, type NavigableElement, type NavigableElementCustomInteractionHandlers } from '../..';

const props = defineProps<NavigableListProps>()

const id = generateNavigableElementId()

const list = computed<NavigableList>(() => ({
    id,
    type: 'list',
    disableScroll: props.disableScroll,
}))

const eventHandlers = computed<NavigableElementCustomInteractionHandlers<'list'>>(() => ({
    navigate(list, currentChild, dir) {
        props.onNavigate?.(dir, currentChild, list)

        return { type: 'native', trap: props.trapped }
    },

    enterFrom(list, from) {
        props.onEnter?.(from, list)
        return { type: 'native' }
    },

    interceptKeyPress(list, key) {
        const dir = key.longPress ? null : translateNavigationKey(key.key)

        if (dir === NavigationDirection.Up) {
            props.onUpKey?.(list)
        } else if (dir === NavigationDirection.Left) {
            props.onLeftKey?.(list)
        } else if (dir === NavigationDirection.Right) {
            props.onRightKey?.(list)
        } else if (dir === NavigationDirection.Down) {
            props.onDownKey?.(list)
        } else if (dir === NavigationDirection.Back) {
            props.onBackKey?.(list)
        }

        return props.interceptKeyPress?.(key.longPress ? null : dir, key, list) ? { type: 'trap' } : { type: 'native' }
    },

    focus(list, focusedChild) {
        focused.value = true
        props.onFocus?.(list, focusedChild)
    },

    unfocus(list, unfocusedChild) {
        focused.value = false
        props.onUnfocus?.(list, unfocusedChild)
    },
}))

onMounted(() => registerNavigableElementHandlers(list.value, eventHandlers.value))
onBeforeUpdate(() => updateNavigableElementHandlers(list.value, eventHandlers.value))
onBeforeUnmount(() => unregisterNavigableElementHandlers(list.value))

const focused = ref(false)

defineExpose({ list, focused })

defineSlots<{
    default(props: { list: NavigableList, focused: boolean }): unknown
}>()
</script>

<template>
    <div class="flex flex-col" v-bind="navigableElementAttrs(list)">
        <slot :list :focused />
    </div>
</template>
