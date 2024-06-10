<script setup lang="ts">
import type { SimpleNavigableItem as SimpleNavigableItemClass } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem';
import SimpleNavigableItem, { type SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';

const props = defineProps<SimpleNavigableItemProps>()

defineSlots<{
    default(props: { item: SimpleNavigableItemClass, focused: boolean }): unknown
}>()
</script>

<template>
    <SimpleNavigableItem v-bind="props" class="p-0 mr-5" not-rounded v-slot="{ item, focused }">
        <div class="flex items-center border border-solid border-white p-5 w-fit"
            :class="{ 'opacity-50': props.disabled }">
            <slot :item :focused />
        </div>
    </SimpleNavigableItem>
</template>
