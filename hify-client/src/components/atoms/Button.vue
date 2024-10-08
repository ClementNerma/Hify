<script lang="ts">
export type ButtonProps = NavigableItemProps & {
    borderNone?: boolean
}

export type ButtonExposeType = { itemRef: NavigableItemExposeType | null }
</script>

<script setup lang="ts">
import NavigableItem, { type NavigableItemExposeType, type NavigableItemProps } from '@/navigable/vue/components/NavigableItem.vue';
import { type NavigableItem as NavigableItemType } from '@/navigable';
import { ref, type Ref } from 'vue';

const props = defineProps<ButtonProps>()

defineSlots<{
    default(props: { item: NavigableItemType, focused: boolean }): unknown
}>()

const itemRef = ref<NavigableItemExposeType | null>(null)

defineExpose({ itemRef })
</script>

<template>
    <NavigableItem class="p-0 mr-5" v-bind="props" v-slot="{ item, focused }" ref="itemRef">
        <div class="items-center w-fit"
            :class="{ 'border': !borderNone, 'border-solid': !borderNone, 'border-white': !borderNone, 'opacity-50': disabled }">
            <slot :item :focused />
        </div>
    </NavigableItem>
</template>
