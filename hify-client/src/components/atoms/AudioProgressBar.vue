<script setup lang="ts">
import { NavigationDirection } from '@/navigable';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';
import { computed, ref } from 'vue';

defineProps<{
    max: number,
    value: number,
    onDirection: (direction: 'left' | 'right') => void,
    onPress?: () => void
}>()

const focused = ref(false)
</script>

<template>
    <NavigableItem :intercept-key-press="dir => dir === NavigationDirection.Left || dir === NavigationDirection.Right"
        @focus="focused = true" @unfocus="focused = false" @left-key="onDirection('left')"
        @right-key="onDirection('right')" :on-press v-slot="{ focused }">
        <div class="border border-solid border-transparent">
            <input type="range" ref="inputRef" class="h-2.5" :max :value :class="{ 'accent-orange-400': focused }" />
        </div>
    </NavigableItem>
</template>
