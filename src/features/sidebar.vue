<script setup lang="ts">
import { computed, ref } from "vue";
import type { Component } from "vue";
import { IconField, InputIcon, InputText, Menu } from "primevue";
import { useDbStore } from "../store/db-store";
import Postgres from "../components/icon/postgres.vue";
import ManageConnectionModal from "./manage-connection-modal.vue";
import { invoke } from "@tauri-apps/api/core";

const searchQuery = ref("");
const dbStore = useDbStore();

const activeConnection = computed(() => dbStore.activeConnection);
const tables = computed(() => dbStore.tables);

const activeSchema = ref("public");
const selectSchemaDropdown = ref();
const schemas = computed(() => [
  {
    label: "Select schema",
    items: dbStore.schemas.map((s) => ({ label: s, value: s })),
    command: (val: string) => (activeSchema.value = val),
  },
]);

const toggleSchemaDropdown = (event) => {
  selectSchemaDropdown.value.toggle(event);
};

const isLoadingFetchTables = ref(false);
const fetchTables = async (schema: string) => {
  try {
    isLoadingFetchTables.value = true;
    const tables = await invoke("get_tables_by_schema_postgres", {
      name: activeConnection.value.name,
      schema,
    });
    activeSchema.value = schema;
    dbStore.setTables(tables);
  } catch (error) {
    console.log(error);
  } finally {
    isLoadingFetchTables.value = false;
  }
};

const icons: Record<string, Component> = {
  postgres: Postgres,
};
</script>

<template>
  <div class="w-64 border-r border-zinc-700 flex flex-col">
    <!-- manage connection toolbar -->
    <div
      class="flex items-center justify-between py-2 px-4 border-b border-zinc-700"
    >
      <div class="flex gap-2 items-center">
        <component :is="icons[activeConnection.type]" class="w-4 h-4" />
        <div class="flex items-center truncate max-w-40">
          <span class="text-sm">{{ activeConnection.name }}</span>
        </div>
      </div>

      <ManageConnectionModal />
    </div>

    <div class="p-4 text-sm flex flex-col justify-between gap-2">
      <IconField>
        <InputIcon class="pi pi-search" />
        <InputText
          size="small"
          v-model="searchQuery"
          placeholder="Search..."
          block
        />
      </IconField>

      <div class="flex justify-between items-center mt-1">
        <div class="flex gap-1 text-xs">
          <span>{{ activeSchema }}</span>
          <span class="bg-zinc-600 px-1 rounded flex items-center">
            {{ schemas[0].items.length }}
          </span>
        </div>

        <div class="flex items-center">
          <span
            class="pi pi-code stroke-[0.5] rotate-90 hover:bg-zinc-600 p-1 rounded cursor-pointer"
            :style="{ fontSize: '8px' }"
            aria-controls="schema_menu"
            aria-haspopup="true"
            @click="toggleSchemaDropdown"
          ></span>
          <Menu
            ref="selectSchemaDropdown"
            id="schema_menu"
            :model="schemas"
            :popup="true"
          >
            <template #item="{ item, props }">
              <span
                class="ml-2 text-xs w-full my-auto"
                @click="() => fetchTables(item.value)"
              >
                {{ item.label }}
              </span>
            </template>
          </Menu>
        </div>
      </div>
    </div>

    <div class="px-4 flex-1 border-b border-zinc-700">
      <div
        v-for="table in tables"
        :key="table"
        class="p-2 text-xs flex gap-2 items-center hover:bg-zinc-600 rounded cursor-pointer"
      >
        <i class="pi pi-table text-emerald-500"></i>
        <span>{{ table }}</span>
      </div>
    </div>

    <!-- Query Section -->
    <div class="mt-auto">
      <div class="px-4 py-2">Query</div>
      <div
        class="px-4 py-2 flex items-center gap-2 hover:bg-zinc-800 cursor-pointer"
      >
        <i class="pi pi-code"></i>
        <span>SQL Query</span>
      </div>
    </div>
  </div>
</template>
