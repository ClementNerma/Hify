<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref } from 'vue'
import {
    generateNavigableElementId,
    type KeyPress,
    type NavigableCommonElementProps,
    type NavigableElement,
    type NavigableElementCustomInteractionHandlers,
    NavigationDirection,
    navigableElementAttrs,
    registerNavigableElementHandlers,
    translateNavigationKey,
    unregisterNavigableElementHandlers,
    updateNavigableElementHandlers,
} from '../..'

// TODO: required because Vue's compiler is not smart enough yet
type NavigableColumn = NavigableCommonElementProps & { type: 'column' }

export type NavigableColumnProps = {
    interceptKeyPress?: (navigationKey: NavigationDirection | null, key: KeyPress, col: NavigableColumn) => boolean

    onFocus?: (col: NavigableColumn, focusedChild: NavigableElement) => void
    onUnfocus?: (col: NavigableColumn, unfocusedChild: NavigableElement) => void
    onNavigate?: (key: NavigationDirection, currentChild: NavigableElement, col: NavigableColumn) => void
    onEnter?: (from: NavigationDirection, col: NavigableColumn) => void
    onLeftKey?: (col: NavigableColumn) => void
    onRightKey?: (col: NavigableColumn) => void
    onUpKey?: (col: NavigableColumn) => void
    onDownKey?: (col: NavigableColumn) => void
    onBackKey?: (col: NavigableColumn) => void

    trapped?: boolean
} & Omit<NavigableColumn, 'id' | 'type'>

export type NavigableColumnExposeType = {
    column: NavigableColumn
    focused: boolean
}

const props = defineProps<NavigableColumnProps>()

const id = generateNavigableElementId()

const column = computed<NavigableColumn>(() => ({
    id,
    type: 'column',
    disableScroll: props.disableScroll,
}))

const eventHandlers = computed<NavigableElementCustomInteractionHandlers<'column'>>(() => ({
    navigate(col, currentChild, dir) {
        props.onNavigate?.(dir, currentChild, col)

        return { type: 'native', trap: props.trapped }
    },

    enterFrom(col, from) {
        props.onEnter?.(from, col)
        return { type: 'native' }
    },

    interceptKeyPress(col, key) {
        const dir = key.longPress ? null : translateNavigationKey(key.key)

        if (dir === NavigationDirection.Up) {
            props.onUpKey?.(col)
        } else if (dir === NavigationDirection.Left) {
            props.onLeftKey?.(col)
        } else if (dir === NavigationDirection.Right) {
            props.onRightKey?.(col)
        } else if (dir === NavigationDirection.Down) {
            props.onDownKey?.(col)
        } else if (dir === NavigationDirection.Back) {
            props.onBackKey?.(col)
        }

        return props.interceptKeyPress?.(key.longPress ? null : dir, key, col) ? { type: 'trap' } : { type: 'native' }
    },

    focus(col, focusedChild) {
        focused.value = true
        props.onFocus?.(col, focusedChild)
    },

    unfocus(col, unfocusedChild) {
        focused.value = false
        props.onUnfocus?.(col, unfocusedChild)
    },
}))

onMounted(() => registerNavigableElementHandlers(column.value, eventHandlers.value))
onBeforeUpdate(() => updateNavigableElementHandlers(column.value, eventHandlers.value))
onBeforeUnmount(() => unregisterNavigableElementHandlers(column.value))

const focused = ref(false)

defineExpose({ column, focused })

defineSlots<{
    default(props: { column: NavigableColumn; focused: boolean }): unknown
}>()
</script>

<template>
    <div
        class="flex flex-col"
        v-bind="navigableElementAttrs(column)"
    >
        <slot
            :column
            :focused
        />
    </div>
</template>
