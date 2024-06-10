<script lang="ts">
export type ButtonProps = SimpleNavigableItemProps & {
    borderNone?: boolean
}
</script>

<script setup lang="ts">
import type { SimpleNavigableItem as SimpleNavigableItemClass } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem';
import SimpleNavigableItem, { type SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';

const props = defineProps<ButtonProps>()

defineSlots<{
    default(props: { item: SimpleNavigableItemClass, focused: boolean }): unknown
}>()
</script>

<template>
    <SimpleNavigableItem v-bind="props" class="p-0 mr-5" not-rounded v-slot="{ item, focused }">
        <div class="flex items-center w-fit"
            :class="{ 'border': !borderNone, 'border-solid': !borderNone, 'border-white': !borderNone, 'opacity-50': disabled }">
            <slot :item :focused />
        </div>
    </SimpleNavigableItem>
</template>
