<script setup lang="ts">
import { ref } from 'vue'
import { NavigationDirection } from '@/navigable'
import NavigableItem, { type NavigableItemProps } from '@/navigable/vue/components/NavigableItem.vue'

const props = defineProps<{
	max: number
	value: number
	onDirection: (direction: 'left' | 'right') => void
	onPress?: () => void
}>()

const interceptKeyPress: NavigableItemProps['interceptKeyPress'] = (dir) => {
	if (dir === NavigationDirection.Left) {
		props.onDirection('left')
		return true
	}

	if (dir === NavigationDirection.Right) {
		props.onDirection('right')
		return true
	}

	return false
}

const focused = ref(false)
</script>

<template>
    <NavigableItem :intercept-key-press :on-press @focus="focused = true" @unfocus="focused = false"
        v-slot="{ focused }">
        <div class="border border-solid border-transparent" v-bind="$attrs">
            <input type="range" class="h-2.5 w-full" :max :value :class="{ 'accent-orange-400': focused }" />
        </div>
    </NavigableItem>
</template>
