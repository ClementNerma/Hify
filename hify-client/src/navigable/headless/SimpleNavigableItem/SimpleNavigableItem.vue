<script lang="ts">
export type SimpleNavigableItemProps = Omit<SimpleNavigableItemClassProps, 'getUnderlyingElement'> & {
    justForStyle?: boolean
    disabled?: boolean
}
</script>

<script setup lang="ts">
import { SimpleNavigableItem, type SimpleNavigableItemProps as SimpleNavigableItemClassProps } from './SimpleNavigableItem';
import { JUST_FOR_STYLE_ITEM_ID, getParentNavigable, type HTMLNavigableItemWrapperElement } from '../../navigation'
import { computed, onBeforeUpdate, ref } from 'vue';
import { logFatal } from '@/global/stores/debugger';

const props = defineProps<SimpleNavigableItemProps>()

defineSlots<{
    default(props: { item: SimpleNavigableItem, focused: boolean }): unknown
}>()

const itemProps = computed(() => ({
    ...props,

    onFocus() {
        focused.value = true
        props.onFocus?.()
    },

    onUnfocus() {
        focused.value = false
        props.onUnfocus?.()
    },

    getUnderlyingElement: () =>
        wrapperRef?.value ?? logFatal('Wrapper reference not initialized yet')

}))

const item = new SimpleNavigableItem(getParentNavigable(true), itemProps.value)

onBeforeUpdate(() => item.updateProps(itemProps.value))

const wrapperRef = ref<HTMLNavigableItemWrapperElement | null>(null)
const focused = ref(false)
</script>

<template>
    <navigable-item-wrapper ref="wrapperRef" :class="{ focused }"
        :data-navigable-item-id="props.justForStyle ? JUST_FOR_STYLE_ITEM_ID : item.id" @click="props.onPress?.()"
        @contextmenu.prevent="props.onLongPress?.()" @mouseenter="item.requestFocus()"
        @mouseleave="item.page.unfocus()">
        <slot :item :focused />
    </navigable-item-wrapper>
</template>
