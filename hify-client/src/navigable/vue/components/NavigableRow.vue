<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableRow = { id: string, type: 'row' }

export type NavigableRowProps = {
    interceptDirectionKeys?: NavigationDirection[]

    display?: CSSProperties['display'],

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
</script>

<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref, type CSSProperties, } from 'vue';
import { NavigationDirection, generateNavigableElementId, navigableElementAttrs, registerNavigableElementHandlers, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type NavigableElement, type NavigableElementCustomInteractionHandlers } from '../..';

const props = defineProps<NavigableRowProps>()

const id = generateNavigableElementId()

const row = computed<NavigableRow>(() => ({
    id,
    type: 'row',
}))

const eventHandlers = computed<NavigableElementCustomInteractionHandlers<'row'>>(() => ({
    navigate(row, currentChild, key) {
        props.onNavigate?.(key, currentChild, row)

        if (key === NavigationDirection.Up) {
            props.onUpKey?.(row)
        } else if (key === NavigationDirection.Left) {
            props.onLeftKey?.(row)
        } else if (key === NavigationDirection.Right) {
            props.onRightKey?.(row)
        } else if (key === NavigationDirection.Down) {
            props.onDownKey?.(row)
        } else if (key === NavigationDirection.Back) {
            props.onBackKey?.(row)
        }

        return props.interceptDirectionKeys?.includes(key) ? { type: 'trap' } : { type: 'native' }
    },

    enterFrom(row, from) {
        props.onEnter?.(from, row)
        return { type: 'native' }
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
