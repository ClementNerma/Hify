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
        <div class="button" :class="{ disabled: props.disabled }">
            <slot :item :focused />
        </div>
    </SimpleNavigableItem>
</template>

<style scoped>
.button {
    display: flex;
    align-items: center;

    border: 1px solid white;

    width: fit-content;
    padding: 5px;
}

.button.disabled {
    opacity: 0.5;
}
</style>