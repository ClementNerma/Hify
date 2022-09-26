<script lang="ts">
  import Button from '../../atoms/Button/Button.svelte'
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import NavigableRow from '../../navigable/NavigableRow/NavigableRow.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { appLogs } from '../../stores/debugger'
  import { enableDistractionFreeModeFeature } from '../../stores/distraction-free'

  let hideDebugLogs = true

  function toggleDebugLogsDisplay() {
    hideDebugLogs = !hideDebugLogs
  }

  function toggleDistractionFreeEnabling() {
    enableDistractionFreeModeFeature.update((set) => !set)
  }

  $: slicedAppLogs = $appLogs.slice(0, 100).filter((entry) => (hideDebugLogs ? entry.level !== 'debug' : true))
</script>

<h2>Developer Tools</h2>

<NavigableRow>
  <Button onPress={() => location.reload()}>Reload the application</Button>
  <Button onPress={toggleDebugLogsDisplay}>
    <input type="checkbox" checked={hideDebugLogs} /> Hide debug logs
  </Button>
  <Button onPress={toggleDistractionFreeEnabling}>
    <input type="checkbox" checked={$enableDistractionFreeModeFeature} /> Enable distraction-free mode
  </Button>
</NavigableRow>

<NavigableList>
  <ul>
    {#each slicedAppLogs as logEntry}
      <SimpleNavigableItem displayBlock={true}>
        <li class="log-entry {logEntry.level}">
          <u>{logEntry.level.toLocaleUpperCase()}</u>
          <strong>{logEntry.at.toLocaleTimeString()}</strong>: {logEntry.message}
        </li>
      </SimpleNavigableItem>
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
