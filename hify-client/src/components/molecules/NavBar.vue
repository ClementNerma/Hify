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
import { requestFocusOnItem } from '@/navigable'
import NavigableItem, { type NavigableItemExposeType } from '@/navigable/vue/components/NavigableItem.vue'
import NavigableRow from '@/navigable/vue/components/NavigableRow.vue'
import { showContextMenu } from '@/global/stores/context-menu'
import { bindRef } from '@/global/utils'

const props = defineProps<{
  tabs: Tab[],
}>()

onMounted(() => {
  router.afterEach((to) => {
    if (typeof to.name === 'string' && Object.prototype.hasOwnProperty.call(routeLinkByName.value, to.name)) {
      requestFocusOnItem(routeLinkByName.value[to.name].item)
    } else {
      // Fallback to first link
      requestFocusOnItem(Object.values(routeLinkByName.value)[0].item)
    }
  })
})

function showSubMenu(subMenu: TabDropdownItem[]) {
  showContextMenu(
    subMenu.map(({ label, routeName }) => ({
      label,
      onPress: () => { router.push({ name: routeName }) },
    })),
  )
}

const routeLinkByName = ref<Record<string, NavigableItemExposeType>>({})

function scrollTop() {
  window.scrollTo({ top: 0, left: 0, behavior: 'smooth' })
}
</script>

<template>
  <NavigableRow @focus="scrollTop()" v-slot="{ focused }">
    <div class="flex flex-row items-center justify-center mb-2.5 w-full gap-10"
      :class="focused ? [] : ['opacity-20', 'transition ease-linear delay-200 duration-700']">
      <NavigableItem v-for="tab in tabs" :key="tab.label" :ref="bindRef(routeLinkByName, tab.routeName)"
        @press="router.push({ name: tab.routeName })" @long-press="tab.subMenu && showSubMenu(tab.subMenu)"
        :has-focus-priority="router.currentRoute.value.name === tab.routeName">
        <div class="!px-6 !py-1">
          {{ tab.label }}
          <span class="text-xs" v-if="tab.subMenu">▽</span>
        </div>
      </NavigableItem>
    </div>
  </NavigableRow>
</template>
