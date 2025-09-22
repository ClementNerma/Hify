<script setup lang="ts">
import { computed, ref } from 'vue'
import Button from '@/components/atoms/Button.vue'
import Checkbox from '@/components/atoms/Checkbox.vue'
import { hifyInterface } from '@/global/injected'
import { appLogs } from '@/global/stores/debugger'
import { LogLevel } from '@/navigable'
import NavigableColumn from '@/navigable/vue/components/NavigableColumn.vue'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue'

const hideDebugLogs = ref(true)

const slicedAppLogs = computed(() =>
	appLogs.value.filter((entry) => (hideDebugLogs.value ? entry.level !== LogLevel.Debug : true)),
)

const win = window
</script>

<template>
  <h2>Developer Tools</h2>

  <NavigableRow>
    <Button @press="win.location.reload()">Reload the application</Button>

    <Checkbox v-model="hideDebugLogs">Hide debug logs</Checkbox>

    <Button
      v-if="hifyInterface"
      @press="hifyInterface?.updateAppUrl()"
    >
      🛠️ Change the application's URL
    </Button>
  </NavigableRow>

  <NavigableColumn>
    <NavigableItem v-for="logEntry in slicedAppLogs">
      <div
        class="log-entry"
        :class="[logEntry.level.toLocaleLowerCase()]"
      >
        <u>{{ logEntry.level.toLocaleUpperCase() }}</u>
        <strong>{{ logEntry.at.toLocaleTimeString() }}</strong>: {{ logEntry.message }}
      </div>
    </NavigableItem>
  </NavigableColumn>
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
