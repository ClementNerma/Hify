<script setup lang="ts">
import { logFatal } from '@/global/stores/debugger';
import type { SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import { ref } from 'vue';

const { onChange, directionalAmount, max } = defineProps<{
    max: number,
    value: number | null,
    onChange: (newValue: number) => void,
    directionalAmount: number,
    onPress?: SimpleNavigableItemProps['onPress']
}>()

const inputRef = ref<HTMLInputElement | null>(null)

function getValue(): number {
    if (!inputRef.value) {
        logFatal('Input reference is not initialized yet')
    }

    const number = Number.parseInt(inputRef.value.value)

    if (Number.isNaN(number)) {
        throw new Error('Range value is not a number!')
    }

    return number
}

function onLeft() {
    onChange(Math.max(getValue() - directionalAmount, 0))
}

function onRight() {
    onChange(Math.min(getValue() + directionalAmount, max))
}
</script>

<template>
    <SimpleNavigableItem :on-left :on-right :on-press v-slot="{ focused }">
        <div class="border border-solid border-transparent" :class="{ 'border-gray-400': focused }">
            <input type="range" ref="inputRef" class="h-2.5" :max :value="value ?? 0" @change="onChange(getValue())" />
        </div>
    </SimpleNavigableItem>
</template>
