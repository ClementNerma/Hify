<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableRow = NavigableCommonElementProps & { type: 'row' }

export type NavigableRowProps = {
    interceptKeyPress?: (navigationKey: NavigationDirection | null, key: string, longPress: boolean, row: NavigableRow) => boolean

    onFocus?: (grid: NavigableRow, focusedChild: NavigableElement) => void,
    onUnfocus?: (grid: NavigableRow, unfocusedChild: NavigableElement) => void,
    onNavigate?: (key: NavigationDirection, currentChild: NavigableElement, row: NavigableRow) => void,
    onEnter?: (from: NavigationDirection, row: NavigableRow) => void,
    onLeftKey?: (row: NavigableRow) => void,
    onRightKey?: (row: NavigableRow) => void,
    onUpKey?: (row: NavigableRow) => void,
    onDownKey?: (row: NavigableRow) => void
    onBackKey?: (row: NavigableRow) => void
} & Omit<NavigableRow, 'id' | 'type'>

export type NavigableRowExposeType = {
    row: NavigableRow
    focused: boolean
}
</script>

<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref } from 'vue';
import { NavigationDirection, generateNavigableElementId, type NavigableCommonElementProps, navigableElementAttrs, registerNavigableElementHandlers, translateNavigationKey, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type NavigableElement, type NavigableElementCustomInteractionHandlers } from '../..';

const props = defineProps<NavigableRowProps>()

const id = generateNavigableElementId()

const row = computed<NavigableRow>(() => ({
    id,
    type: 'row',
    disableScroll: props.disableScroll
}))

const eventHandlers = computed<NavigableElementCustomInteractionHandlers<'row'>>(() => ({
    navigate(row, currentChild, dir) {
        props.onNavigate?.(dir, currentChild, row)

        return { type: 'native' }
    },

    enterFrom(row, from) {
        props.onEnter?.(from, row)
        return { type: 'native' }
    },

    interceptKeyPress(row, key, longPress, currentlyFocusedChild) {
        const dir = longPress ? null : translateNavigationKey(key)

        if (dir === NavigationDirection.Up) {
            props.onUpKey?.(row)
        } else if (dir === NavigationDirection.Left) {
            props.onLeftKey?.(row)
        } else if (dir === NavigationDirection.Right) {
            props.onRightKey?.(row)
        } else if (dir === NavigationDirection.Down) {
            props.onDownKey?.(row)
        } else if (dir === NavigationDirection.Back) {
            props.onBackKey?.(row)
        }

        return props.interceptKeyPress?.(longPress ? null : dir, key, longPress, row) ? { type: 'trap' } : { type: 'native' }
    },

    focus(row, focusedChild) {
        focused.value = true
        props.onFocus?.(row, focusedChild)
    },

    unfocus(row, unfocusedChild) {
        focused.value = false
        props.onUnfocus?.(row, unfocusedChild)
    },
}))

onMounted(() => registerNavigableElementHandlers(row.value, eventHandlers.value))
onBeforeUpdate(() => updateNavigableElementHandlers(row.value, eventHandlers.value))
onBeforeUnmount(() => unregisterNavigableElementHandlers(row.value))

const focused = ref(false)

defineExpose({ row, focused })

defineSlots<{
    default(props: { row: NavigableRow, focused: boolean }): unknown
}>()
</script>

<template>
    <div class="flex flex-row" v-bind="navigableElementAttrs(row)">
        <slot :row :focused />
    </div>
</template>
