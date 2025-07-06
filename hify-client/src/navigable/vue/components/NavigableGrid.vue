<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableGrid = NavigableCommonElementProps & { type: 'grid'; columns: number }

export type NavigableGridProps = {
	interceptKeyPress?: (navigationKey: NavigationDirection | null, key: KeyPress, grid: NavigableGrid) => boolean

	onFocus?: (grid: NavigableGrid, focusedChild: NavigableElement) => void
	onUnfocus?: (grid: NavigableGrid, unfocusedChild: NavigableElement) => void
	onNavigate?: (key: NavigationDirection, currentChild: NavigableElement, grid: NavigableGrid) => void
	onEnter?: (from: NavigationDirection, grid: NavigableGrid) => void
	onLeftKey?: (grid: NavigableGrid) => void
	onRightKey?: (grid: NavigableGrid) => void
	onUpKey?: (grid: NavigableGrid) => void
	onDownKey?: (grid: NavigableGrid) => void
	onBackKey?: (grid: NavigableGrid) => void
} & Omit<NavigableGrid, 'id' | 'type'>

export type NavigableGridExposeType = {
	grid: NavigableGrid
	focused: boolean
}
</script>

<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref } from 'vue';
import { NavigationDirection, generateNavigableElementId, navigableElementAttrs, registerNavigableElementHandlers, translateNavigationKey, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type KeyPress, type NavigableCommonElementProps, type NavigableElement, type NavigableElementCustomInteractionHandlers } from '../..';

const props = defineProps<NavigableGridProps>()

const id = generateNavigableElementId()

const grid = computed<NavigableGrid>(() => ({
    id,
    type: 'grid',
    disableScroll: props.disableScroll,
    columns: props.columns
}))

const eventHandlers = computed<NavigableElementCustomInteractionHandlers<'grid'>>(() => ({
    navigate(grid, currentChild, dir) {
        props.onNavigate?.(dir, currentChild, grid)

        return { type: 'native' }
    },

    enterFrom(grid, from) {
        props.onEnter?.(from, grid)
        return { type: 'native' }
    },

    interceptKeyPress(grid, key) {
        const dir = key.longPress ? null : translateNavigationKey(key.key)

        if (dir === NavigationDirection.Up) {
            props.onUpKey?.(grid)
        } else if (dir === NavigationDirection.Left) {
            props.onLeftKey?.(grid)
        } else if (dir === NavigationDirection.Right) {
            props.onRightKey?.(grid)
        } else if (dir === NavigationDirection.Down) {
            props.onDownKey?.(grid)
        } else if (dir === NavigationDirection.Back) {
            props.onBackKey?.(grid)
        }

        return props.interceptKeyPress?.(key.longPress ? null : dir, key, grid) ? { type: 'trap' } : { type: 'native' }
    },

    focus(grid, focusedChild) {
        focused.value = true
        props.onFocus?.(grid, focusedChild)
    },

    unfocus(grid, unfocusedChild) {
        focused.value = false
        props.onUnfocus?.(grid, unfocusedChild)
    },
}))

onMounted(() => registerNavigableElementHandlers(grid.value, eventHandlers.value))
onBeforeUpdate(() => updateNavigableElementHandlers(grid.value, eventHandlers.value))
onBeforeUnmount(() => unregisterNavigableElementHandlers(grid.value))

const focused = ref(false)

defineExpose({ grid, focused })

defineSlots<{
    default(props: { grid: NavigableGrid, focused: boolean }): unknown
}>()
</script>

<template>
    <div class="container grid text-center min-w-full" v-bind="navigableElementAttrs(grid)">
        <slot :grid :focused />
    </div>
</template>

<style scoped>
.container {
    grid-template-columns: repeat(v-bind(columns), 1fr);
}
</style>
