<script lang="ts">
export type SimpleNavigableItemProps = Omit<SimpleNavigableItemClassProps, 'getUnderlyingElement'> & NavigableCommonProps & {
    justForStyle?: boolean
    disabled?: boolean
    fullHeight?: boolean
    unstyled?: boolean
}
</script>

<script setup lang="ts">
import { SimpleNavigableItem, type SimpleNavigableItemProps as SimpleNavigableItemClassProps } from './SimpleNavigableItem';
import { JUST_FOR_STYLE_ITEM_ID, getParentNavigable, type HTMLNavigableItemWrapperElement, type NavigableCommonProps } from '../../navigation'
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

defineExpose({
    requestFocus: () => item.requestFocus()
})

const wrapperRef = ref<HTMLNavigableItemWrapperElement | null>(null)
const focused = ref(false)
</script>

<template>
    <navigable-item-wrapper ref="wrapperRef" :class="{ focused, fullHeight, unstyled }"
        :data-navigable-item-id="props.justForStyle ? JUST_FOR_STYLE_ITEM_ID : item.id" @click="props.onPress?.()"
        @contextmenu.prevent="props.onLongPress?.()" @mouseenter="item.requestFocus()"
        @mouseleave="item.page.unfocus()">
        <slot :item :focused />
    </navigable-item-wrapper>
</template>

<style scoped>
navigable-item-wrapper.fullHeight {
    /* -12px for the 2x 6px padding */
    height: calc(100% - 12px);
}
</style>
