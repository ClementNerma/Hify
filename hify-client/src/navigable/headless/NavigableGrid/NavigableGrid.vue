<script setup lang="ts">
import { getParentNavigable } from '@/navigable/navigation';
import { NavigableGrid, type NavigableGridProps } from './NavigableGrid';
import InternalNavWrapper from '../InternalNavWrapper.vue';

const props = defineProps<NavigableGridProps>()

const grid = new NavigableGrid(getParentNavigable(), props)

defineSlots<{
    default(props: { grid: NavigableGrid }): unknown
}>()
</script>

<template>
    <InternalNavWrapper :nav="grid" :props>
        <slot :grid />
    </InternalNavWrapper>
</template>