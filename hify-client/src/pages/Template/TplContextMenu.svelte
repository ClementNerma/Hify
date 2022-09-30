<script context="module" lang="ts">
  import { createContextMenu } from '../../molecules/ContextMenu/context-menu'
  import ContextMenu from '../../molecules/ContextMenu/ContextMenu.svelte'
  import { logFatal } from '../../stores/debugger'

  export const contextMenuStore = createContextMenu()

  let _singletonChecker = false
</script>

<script lang="ts">
  if (_singletonChecker) {
    logFatal('Cannot instanciate a TplContextMenu component twice!')
  }

  _singletonChecker = true
</script>

<ContextMenu store={contextMenuStore} />
