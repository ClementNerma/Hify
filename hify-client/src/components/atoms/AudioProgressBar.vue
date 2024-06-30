<script setup lang="ts">
import { NavigationDirection } from '@/navigable';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';

defineProps<{
    max: number,
    value: number,
    onDirection: (direction: 'left' | 'right') => void,
    onPress?: () => void
}>()
</script>

<template>
    <NavigableItem :intercept-key-press="dir => dir === NavigationDirection.Left || dir === NavigationDirection.Right"
        @left-key="onDirection('left')" @right-key="onDirection('right')" :on-press v-slot="{ focused }">
        <div class="border border-solid border-transparent" :class="{ 'border-gray-400': focused }">
            <input type="range" ref="inputRef" class="h-2.5" :max :value />
        </div>
    </NavigableItem>
</template>
