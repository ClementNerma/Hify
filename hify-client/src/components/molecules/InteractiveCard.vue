<script setup lang="ts">
import type { ArtFragment } from '@/graphql/generated/graphql'
import type { SimpleNavigableItemProps } from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';
import Card from './Card.vue';

// TODO: inherit some props from 'Card' and from 'SimpleNavigableItem'
defineProps<{
    art: ArtFragment | null | undefined,
    title: string,

    subtitle?: string,
    onPress: SimpleNavigableItemProps['onPress'],

    onLongPress?: SimpleNavigableItemProps['onLongPress'],
    onFocus?: SimpleNavigableItemProps['onFocus'],

    enforceMaxWidth?: boolean,
    circle?: boolean
}>()
</script>

<template>
    <SimpleNavigableItem :on-press :on-long-press :on-focus>
        <Card :art :title :subtitle :enforce-max-width :circle />
    </SimpleNavigableItem>
</template>
