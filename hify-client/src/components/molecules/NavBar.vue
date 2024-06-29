<script lang="ts">
export type Tab = {
  label: string
  routeName: string
  subMenu?: TabDropdownItem[]
}

export type TabDropdownItem = Omit<Tab, 'subMenu'>
</script>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import router from '@/router'
import Run from '../atoms/Run.vue'
import { requestFocusById } from '@/navigable'
import NavigableItem from '@/navigable/vue/components/NavigableItem.vue'
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue'
import { showContextMenu } from '@/global/stores/context-menu'

const { tabs } = defineProps<{
  tabs: Tab[],
}>()

onMounted(() =>
  router.afterEach((to) => {
    if (typeof to.name === 'string' && Object.prototype.hasOwnProperty.call(linkIdByRouteName.value, to.name)) {
      requestFocusById(linkIdByRouteName.value[to.name])
    } else {
      // Fallback to first tab if needed
      requestFocusById(Object.values(linkIdByRouteName.value)[0])
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

const linkIdByRouteName = ref<Record<string, string>>({})
const isFocused = ref(false)

const win = window
</script>

<template>
  <NavigableRow>
    <div class="flex flex-row items-center justify-center mb-2.5 w-full"
      :class="isFocused ? [] : ['opacity-20', 'transition ease-linear delay-200 duration-700']">
      <NavigableItem v-for="tab in tabs" :key="tab.label" @press="router.push({ name: tab.routeName })"
        @long-press="tab.subMenu && showSubMenu(tab.subMenu)"
        @focus="win.scrollTo({ top: 0, left: 0, behavior: 'smooth' }); isFocused = true" @unfocus="isFocused = false"
        :has-focus-priority="router.currentRoute.value.name === tab.routeName" v-slot="{ item }">

        <!-- TODO: simple "ref" binding from Item instead? -->
        <Run @run="linkIdByRouteName[tab.routeName] = item.id" />

        <div>
          <div class="px-6">
            {{ tab.label }}
            <span class="text-xs" v-if="tab.subMenu">▽</span>
          </div>
        </div>
      </NavigableItem>
    </div>
  </NavigableRow>
</template>
