<script setup lang="ts">
import type { MixParams } from '@/graphql/generated/graphql';
import Button from './Button.vue';
import { generateAndPlayMix } from '@/global/stores/play-queue';
import { showContextMenu } from '@/navigable/ui/molecules/ContextMenu/ContextMenu';

defineProps<{
    mixParams: MixParams
}>()
</script>

<template>
    <Button @press="generateAndPlayMix(mixParams)" @long-press="showContextMenu([
        {
            label: 'Only with the best rating <3',
            onPress: () => void generateAndPlayMix({ ...mixParams, minRating: 10 }),
        }
    ])">
        Mix me some magic ✨
    </Button>
</template>