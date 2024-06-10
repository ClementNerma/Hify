<script lang="ts">
export type Tab = {
  label: string
  routeName: string
  subMenu?: TabDropdownItem[]
}

export type TabDropdownItem = Omit<Tab, 'subMenu'>
</script>

<script setup lang="ts">
import NavigableRow from '@/navigable/headless/NavigableRow/NavigableRow.vue'
import type { RequestFocus } from '@/navigable/navigation'
import SimpleNavigableItem from '@/navigable/headless/SimpleNavigableItem/SimpleNavigableItem.vue'
import { showContextMenu } from '@/navigable/ui/molecules/ContextMenu/ContextMenu'
import { onMounted, ref } from 'vue'
import router from '@/router'
import Run from '../atoms/Run.vue'

const { tabs } = defineProps<{
  tabs: Tab[],
}>()

onMounted(() =>
  router.afterEach((to) => {
    if (typeof to.name === 'string' && Object.prototype.hasOwnProperty.call(requestFocusByRouteName.value, to.name)) {
      requestFocusByRouteName.value[to.name]()
    } else {
      // Fallback to first tab if needed
      Object.values(requestFocusByRouteName.value)[0]()
    }
  })
)

function showSubMenu(subMenu: TabDropdownItem[]) {
  showContextMenu(
    subMenu.map(({ label, routeName }) => ({
      label,
      onPress: () => { router.push({ name: routeName }) },
    })),
  )
}

const requestFocusByRouteName = ref<Record<string, RequestFocus>>({})
const isFocused = ref(false)

const win = window
</script>

<template>
  <div class="container" :class="{ isFocused }">
    <NavigableRow>
      <!-- TODO: bind:requestFocus={tabsFocusRequest[tab.label]} -->
      <SimpleNavigableItem v-for="tab in tabs" :key="tab.label" @press="router.push({ name: tab.routeName })"
        @long-press="tab.subMenu && showSubMenu(tab.subMenu)"
        @focus="win.scrollTo({ top: 0, left: 0, behavior: 'smooth' }); isFocused = true" @unfocus="isFocused = false"
        :has-focus-priority="router.currentRoute.value.name === tab.routeName" v-slot="{ item }">

        <Run :run="() => { requestFocusByRouteName[tab.routeName] = () => item.requestFocus(); }" />

        <div>
          <div class="tab">
            {{ tab.label }}
            <span class="dropdown" v-if="tab.subMenu">▽</span>
          </div>
        </div>
      </SimpleNavigableItem>
    </NavigableRow>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  margin-bottom: 10px;
}

.container:not(.isFocused) {
  opacity: 0.2;
  transition: opacity linear 0.8s;
  transition-delay: 0.2s;
}

.tab {
  padding: 0 25px;
  font-size: 1rem;
}

.dropdown {
  font-size: 8px;
}
</style>
