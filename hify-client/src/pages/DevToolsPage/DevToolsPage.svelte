<script lang="ts">
  import NavigableList from '../../navigable/NavigableList/NavigableList.svelte'
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { appLogs } from '../../stores/debugger'

  $: slicedAppLogs = $appLogs.slice(0, 100)
</script>

<h2>Developer Tools</h2>

<SimpleNavigableItem onPress={() => location.reload()}>
  <button>Reload the application</button>
</SimpleNavigableItem>

<NavigableList>
  <ul>
    {#each slicedAppLogs as logEntry}
      <SimpleNavigableItem>
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
  }

  .log-entry.warn {
    background-color: yellow;
  }

  .log-entry.error {
    background-color: red;
    color: white;
  }
</style>
