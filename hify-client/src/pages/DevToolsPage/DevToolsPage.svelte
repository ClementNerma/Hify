<script lang="ts">
  import SimpleNavigableItem from '../../navigable/SimpleNavigableItem/SimpleNavigableItem.svelte'
  import { appLogs } from '../../stores/audio/debugger'
</script>

<h2>Developer Tools</h2>

<SimpleNavigableItem onPress={() => location.reload()}>
  <button>Reload the application</button>
</SimpleNavigableItem>

<ul>
  {#each $appLogs as logEntry}
    <li><strong>{logEntry.at.toLocaleDateString()}</strong>: {logEntry.message}</li>
  {/each}
</ul>
