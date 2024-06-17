<script lang="ts">
export type CardProps = {
    artType: 'album' | 'artist',
    artItem: { id: string, hasArt: boolean },
    title: string,

    subtitle?: string,
    boxSize?: number,
    circle?: boolean,
    opacity?: number,
}
</script>

<script setup lang="ts">
import ImgLoader from '../atoms/ImgLoader.vue'

defineProps<CardProps>()
</script>

<template>
    <div class="card" :style="`--opacity: ${opacity ?? 1};`">
        <ImgLoader :type="artType" :item="artItem" v-slot="{ src }">
            <img class="cover" :class="{ 'rounded-[50%]': circle }" :width="boxSize ?? 120" :height="boxSize ?? 120"
                :src />
        </ImgLoader>

        <div class="title experimental-line-limiter">{{ title }}</div>

        <div v-if="subtitle" class="text-sm experimental-line-limiter">{{ subtitle }}</div>
    </div>

</template>

<style scoped>
.card {
    text-align: center;
    transition: transform 0.25s;
    opacity: var(--opacity);
}

.experimental-line-limiter {
    overflow: hidden;
    text-overflow: ellipsis;

    /* HACK: Limit the number of lines using deprecated CSS properties
       Unfortunately, it is currently impossible to do this using standard CSS :( */
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
}
</style>
