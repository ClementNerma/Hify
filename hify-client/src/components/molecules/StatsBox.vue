<script setup lang="ts">
import { humanReadableDuration } from '@/global/stores/audio-player';
import { UsageStatsDocument } from '@/graphql/generated/graphql';
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue';
import { useQuery } from '@urql/vue';
import { computed } from 'vue';

const { data, fetching } = useQuery({
    query: UsageStatsDocument,
})

const stats = computed(() => data.value?.generateStats)
</script>

<template>
    <SimpleNavigableItem>
        <span v-if="fetching"><strong>Loading...</strong></span>

        <table v-if="stats">
            <tr>
                <td>Total number of tracks</td>
                <td><strong>{{ stats.tracksCount }}</strong></td>
            </tr>
            <tr>
                <td>Total number of albums</td>
                <td><strong>{{ stats.albumsCount }}</strong></td>
            </tr>
            <tr>
                <td>Total number of album artists</td>
                <td><strong>{{ stats.albumArtistsCount }}</strong></td>
            </tr>
            <tr>
                <td>Total number of artists</td>
                <td><strong>{{ stats.artistsCount }}</strong></td>
            </tr>
            <tr>
                <td>Number of listened tracks</td>
                <td><strong>{{ stats.totalTracksListened }}</strong></td>
            </tr>
            <tr>
                <td>Total listening duration</td>
                <td><strong>{{ humanReadableDuration(stats.totalListeningTime) }}</strong></td>
            </tr>
        </table>
    </SimpleNavigableItem>
</template>