<script setup lang="ts">
import { NotificationLevel, pendingNotifications } from '@/global/stores/notifications';
</script>

<template>
    <div class="fixed bottom-5 right-5 z-20 w-1/4 items-end flex flex-col gap-5 text-lg">
        <div v-for="notification in pendingNotifications" class="rounded w-full"
            :class="notification.level === NotificationLevel.Error ? 'bg-red-600' : notification.level === NotificationLevel.Warn ? 'bg-yellow-600' : 'bg-blue-600'">
            <p class="m-4 whitespace-pre-wrap">{{ notification.message }}</p>
        </div>
    </div>
</template>
