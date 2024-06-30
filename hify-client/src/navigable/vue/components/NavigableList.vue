<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableList = { id: string, type: 'list' }

export type NavigableListProps = {
    display?: CSSProperties['display'],

    interceptKeyPress?: (navigationKey: NavigationDirection | null, key: string, longPress: boolean, list: NavigableList) => boolean

    onFocus?: (list: NavigableList, focusedChild: NavigableElement) => void,
    onUnfocus?: (list: NavigableList, unfocusedChild: NavigableElement) => void,
    onNavigate?: (key: NavigationDirection, currentChild: NavigableElement, list: NavigableList) => void,
    onEnter?: (from: NavigationDirection, list: NavigableList) => void,
    onLeftKey?: (list: NavigableList) => void,
    onRightKey?: (list: NavigableList) => void,
    onUpKey?: (list: NavigableList) => void,
    onDownKey?: (list: NavigableList) => void
    onBackKey?: (list: NavigableList) => void
} & Omit<NavigableList, 'id' | 'type'>
</script>

<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref, type CSSProperties, } from 'vue';
import { NavigationDirection, generateNavigableElementId, navigableElementAttrs, registerNavigableElementHandlers, translateNavigationKey, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type NavigableElement, type NavigableElementCustomInteractionHandlers } from '../..';

const props = defineProps<NavigableListProps>()

const id = generateNavigableElementId()

const list = computed<NavigableList>(() => ({
    id,
    type: 'list',
}))

const eventHandlers = computed<NavigableElementCustomInteractionHandlers<'list'>>(() => ({
    navigate(list, currentChild, key) {
        props.onNavigate?.(key, currentChild, list)

        if (key === NavigationDirection.Up) {
            props.onUpKey?.(list)
        } else if (key === NavigationDirection.Left) {
            props.onLeftKey?.(list)
        } else if (key === NavigationDirection.Right) {
            props.onRightKey?.(list)
        } else if (key === NavigationDirection.Down) {
            props.onDownKey?.(list)
        } else if (key === NavigationDirection.Back) {
            props.onBackKey?.(list)
        }

        return { type: 'native' }
    },

    enterFrom(list, from) {
        props.onEnter?.(from, list)
        return { type: 'native' }
    },

    interceptKeyPress(navEl, key, longPress, currentlyFocusedChild) {
        return props.interceptKeyPress?.(longPress ? null : translateNavigationKey(key), key, longPress, navEl) ? { type: 'trap' } : { type: 'native' }
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
