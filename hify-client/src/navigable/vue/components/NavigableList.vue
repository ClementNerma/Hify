<script lang="ts">
// TODO: required because Vue's compiler is not smart enough yet
type NavigableList = { id: string, type: 'list' }

export type NavigableListProps = {
    interceptDirectionKeys?: NavigationDirection[]

    display?: CSSProperties['display'],

    onFocus?: (list: NavigableList) => void,
    onUnfocus?: (list: NavigableList) => void,
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
import { NavigationDirection, generateNavigableElementId, navigableElementAttrs, registerNavigableElementHandlers, unregisterNavigableElementHandlers, updateNavigableElementHandlers, type NavigableElement, type NavigableElementCustomInteractionHandlers } from '../..';

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

        return props.interceptDirectionKeys?.includes(key) ? { type: 'trap' } : { type: 'native' }
    },

    enterFrom(list, from) {
        props.onEnter?.(from, list)
        return { type: 'native' }
    },

    focus(list) {
        focused.value = true
        props.onFocus?.(list)
    },

    unfocus(list) {
        focused.value = false
        props.onUnfocus?.(list)
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
