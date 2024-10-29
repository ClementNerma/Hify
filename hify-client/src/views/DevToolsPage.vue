<script setup lang="ts">
import Button from '@/components/atoms/Button.vue';
import Checkbox from '@/components/atoms/Checkbox.vue';
import { hifyInterface } from '@/global/injected';
import { appLogs } from '@/global/stores/debugger';
import { LogLevel } from '@/navigable';
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue';
import NavigableList from '@/navigable/vue/components/NavigableList.vue';
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue';
import { computed, ref } from 'vue'

const hideDebugLogs = ref(true)

const slicedAppLogs = computed(() => appLogs.value.filter((entry) => (hideDebugLogs.value ? entry.level !== LogLevel.Debug : true)))

const win = window
</script>

<template>
  <h2>Developer Tools</h2>

  <NavigableRow>
    <Button @press="win.location.reload()" full-height>Reload the application</Button>

    <Checkbox v-model="hideDebugLogs" full-height>Hide debug logs</Checkbox>

    <Button v-if="hifyInterface" @press="hifyInterface?.updateAppUrl()" full-height>
      🛠️ Change the application's URL
    </Button>
  </NavigableRow>

  <NavigableList>
    <NavigableItem v-for="logEntry in slicedAppLogs">
      <div class="log-entry" :class="[logEntry.level.toLocaleLowerCase()]">
        <u>{{ logEntry.level.toLocaleUpperCase() }}</u>
        <strong>{{ logEntry.at.toLocaleTimeString() }}</strong>: {{ logEntry.message }}
      </div>
    </NavigableItem>
  </NavigableList>
</template>

<style scoped>
.log-entry.info {
  background-color: lightblue;
  color: black;
}

.log-entry.warn {
  background-color: yellow;
  color: black;
}

.log-entry.error {
  background-color: red;
  color: white;
}
</style>
