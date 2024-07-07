<script lang="ts">
export type ButtonProps = NavigableItemProps & {
    borderNone?: boolean
}
</script>

<script setup lang="ts">
import NavigableItem, { type NavigableItemProps } from '@/navigable/vue/components/NavigableItem.vue';
import { type NavigableItem as NavigableItemType } from '@/navigable';
import { ref } from 'vue';

const props = defineProps<ButtonProps>()

defineSlots<{
    default(props: { item: NavigableItemType, focused: boolean }): unknown
}>()

const itemRef = ref<InstanceType<typeof NavigableItem> | null>(null)

defineExpose({ itemRef })
</script>

<template>
    <NavigableItem class="p-0 mr-5" v-bind="props" v-slot="{ item, focused }" ref="itemRef">
        <div class="flex items-center w-fit"
            :class="{ 'border': !borderNone, 'border-solid': !borderNone, 'border-white': !borderNone, 'opacity-50': disabled }">
            <slot :item :focused />
        </div>
    </NavigableItem>
</template>

<style scoped>
div {
    /* Prevent overflows in buttons */
    white-space: nowrap;
}
</style>