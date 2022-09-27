<script lang="ts">
  export let alone = false
</script>

<span class="emoji" class:alone>
  <slot />
</span>

<style>
  .emoji:not(.alone) {
    margin-right: 1px;
  }
</style>
