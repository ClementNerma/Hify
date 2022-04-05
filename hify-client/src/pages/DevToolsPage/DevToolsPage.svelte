<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { appLogs } from '../../stores/audio/debugger'

  $: slicedAppLogs = $appLogs.slice(0, 100)
</script>

<h2>Developer Tools</h2>

<SimpleNavigableItem onPress={() => location.reload()}>
  <button>Reload the application</button>
</SimpleNavigableItem>

<ul>
  {#each slicedAppLogs as logEntry}
    <li class="log-entry {logEntry.level}">
      <u>{logEntry.level.toLocaleUpperCase()}</u>
      <strong>{logEntry.at.toLocaleTimeString()}</strong>: {logEntry.message}
    </li>
  {/each}
</ul>

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
