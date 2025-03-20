<script setup lang="ts">
import { NavigationDirection } from '@/navigable'
import NavigableItem, { type NavigableItemProps } from '@/navigable/vue/components/NavigableItem.vue'
import { ref } from 'vue'

const props = defineProps<{
    max: number
    value: number
    onDirection: (direction: 'left' | 'right', shiftKey: boolean) => void
    onPress?: () => void
}>()

const interceptKeyPress: NavigableItemProps['interceptKeyPress'] = (dir, _, __, { shiftKey }) => {
    if (dir === NavigationDirection.Left) {
        props.onDirection('left', shiftKey)
        return true
    }

    if (dir === NavigationDirection.Right) {
        props.onDirection('right', shiftKey)
        return true
    }

    return false
}

const focused = ref(false)
</script>

<template>
    <NavigableItem :intercept-key-press @focus="focused = true" @unfocus="focused = false" :on-press
        v-slot="{ focused }">
        <div class="border border-solid border-transparent" v-bind="$attrs">
            <input type="range" class="h-2.5 w-full" :max :value :class="{ 'accent-orange-400': focused }" />
        </div>
    </NavigableItem>
</template>
