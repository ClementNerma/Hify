<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableGrid = { id: string, type: 'grid', columns: number }

export type NavigableGridProps = {
    interceptDirectionKeys?: NavigationDirection[]

    onFocus?: (grid: NavigableGrid) => void,
    onUnfocus?: (grid: NavigableGrid) => void,
    onNavigate?: (key: NavigationDirection, currentChild: NavigableElement, grid: NavigableGrid) => void,
    onEnter?: (from: NavigationDirection, grid: NavigableGrid) => void,
    onLeftKey?: (grid: NavigableGrid) => void,
    onRightKey?: (grid: NavigableGrid) => void,
    onUpKey?: (grid: NavigableGrid) => void,
    onDownKey?: (grid: NavigableGrid) => void
    onBackKey?: (grid: NavigableGrid) => void
} & Omit<NavigableGrid, 'id' | 'type'>
</script>

<script setup lang="ts">
import { computed, onBeforeUnmount, onBeforeUpdate, onMounted, ref, } from 'vue';
import { NavigationDirection, generateNavigableElementId, navigableElementAttrs, registerNavigableElementHandlers, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type NavigableElement, type NavigableElementCustomInteractionHandlers } from '../..';

const props = defineProps<NavigableGridProps>()

const id = generateNavigableElementId()

const grid = computed<NavigableGrid>(() => ({
    id,
    type: 'grid',
    columns: props.columns
}))

const eventHandlers = computed<NavigableElementCustomInteractionHandlers<'grid'>>(() => ({
    navigate(grid, currentChild, key) {
        props.onNavigate?.(key, currentChild, grid)

        if (key === NavigationDirection.Up) {
            props.onUpKey?.(grid)
        } else if (key === NavigationDirection.Left) {
            props.onLeftKey?.(grid)
        } else if (key === NavigationDirection.Right) {
            props.onRightKey?.(grid)
        } else if (key === NavigationDirection.Down) {
            props.onDownKey?.(grid)
        } else if (key === NavigationDirection.Back) {
            props.onBackKey?.(grid)
        }

        return props.interceptDirectionKeys?.includes(key) ? { type: 'trap' } : { type: 'native' }
    },

    enterFrom(grid, from) {
        props.onEnter?.(from, grid)
        return { type: 'native' }
    },

    focus(grid) {
        focused.value = true
        props.onFocus?.(grid)
    },

    unfocus(grid) {
        focused.value = false
        props.onUnfocus?.(grid)
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
