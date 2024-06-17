<script setup lang="ts">
import type { SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem';
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'

defineProps<{
    max: number,
    value: number | null,
    onDirection: (direction: 'left' | 'right') => void,
    onPress?: SimpleNavigableItemProps['onPress']
}>()
</script>

<template>
    <SimpleNavigableItem @left="onDirection('left')" @right="onDirection('right')" :on-press v-slot="{ focused }">
        <div class="border border-solid border-transparent" :class="{ 'border-gray-400': focused }">
            <input type="range" ref="inputRef" class="h-2.5" :max :value="value ?? 0" />
        </div>
    </SimpleNavigableItem>
</template>
