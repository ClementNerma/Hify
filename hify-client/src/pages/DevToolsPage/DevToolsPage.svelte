<script lang="ts">
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { appLogs } from '../../stores/debugger'

  $: slicedAppLogs = $appLogs.slice(0, 100)

  let hideDebugLogs = true

  function toggleDebugLogsDisplay() {
    hideDebugLogs = !hideDebugLogs
  }
</script>

<h2>Developer Tools</h2>

<NavigableRow>
  <SimpleNavigableItem onPress={() => location.reload()}>
    <button>Reload the application</button>
  </SimpleNavigableItem>
  <SimpleNavigableItem onPress={toggleDebugLogsDisplay}>
    <input type="checkbox" checked={hideDebugLogs} /> Hide debug logs
  </SimpleNavigableItem>
</NavigableRow>

<NavigableList>
  <ul>
    {#each slicedAppLogs as logEntry}
      {#if logEntry.level !== 'debug' || !hideDebugLogs}
        <SimpleNavigableItem displayBlock={true}>
          <li class="log-entry {logEntry.level}">
            <u>{logEntry.level.toLocaleUpperCase()}</u>
            <strong>{logEntry.at.toLocaleTimeString()}</strong>: {logEntry.message}
          </li>
        </SimpleNavigableItem>
      {/if}
    {/each}
  </ul>
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
