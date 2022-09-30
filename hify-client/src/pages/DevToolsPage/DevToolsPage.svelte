<script lang="ts">
  import Button from '../../atoms/Button/Button.svelte'
  import Checkbox from '../../atoms/Checkbox/Checkbox.svelte'
  import Row from '../../atoms/Row/Row.svelte'
  import { hifyInterface } from '../../globals/injected'
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { appLogs } from '../../stores/debugger'
  import { bind } from '../../globals/utils'

  let hideDebugLogs = true

  $: slicedAppLogs = $appLogs.slice(0, 100).filter((entry) => (hideDebugLogs ? entry.level !== 'debug' : true))
</script>

<h2>Developer Tools</h2>

<Row>
  <Button onPress={() => location.reload()} fullHeight>Reload the application</Button>
  <Checkbox bind:checked={hideDebugLogs} fullHeight>Hide debug logs</Checkbox>
  {#if hifyInterface}
    <Button onPress={bind(hifyInterface, (h) => h.updateAppUrl())} fullHeight>🛠️ Change the application's URL</Button>
  {/if}
</Row>

<NavigableList>
  {#each slicedAppLogs as logEntry}
    <SimpleNavigableItem display="block">
      <div class="log-entry {logEntry.level}">
        <u>{logEntry.level.toLocaleUpperCase()}</u>
        <strong>{logEntry.at.toLocaleTimeString()}</strong>: {logEntry.message}
      </div>
    </SimpleNavigableItem>
  {/each}
</NavigableList>

<style>
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
