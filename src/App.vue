<script setup lang="ts">
import { computed, ref } from "vue";
import { IconField, InputIcon, InputText } from "primevue";
import ManageConnectionModal from "./features/manage-connection-modal.vue";
import { useDbStore } from "./store/db-store";

const searchQuery = ref("");
const dbStore = useDbStore();

const activeConnection = computed(() => dbStore.activeConnection);
const tables = computed(() => dbStore.tables);
</script>

<template>
  <div class="h-screen">
    <div class="flex h-full">
      <!-- Sidebar -->
      <div class="w-64 border-r border-zinc-700 flex flex-col">
        <!-- manage connection toolbar -->
        <div
          class="flex items-center justify-between pl-4 border-b border-zinc-700"
        >
          <div class="flex items-center gap-2 truncate">
            <span class="text-sm">{{ activeConnection.name }}</span>
          </div>

          <ManageConnectionModal />
        </div>

        <div
          class="p-4 text-gray-400 text-sm flex flex-col justify-between gap-2"
        >
          <IconField>
            <InputIcon class="pi pi-search" />
            <InputText
              size="small"
              v-model="searchQuery"
              placeholder="Search..."
              block
            />
          </IconField>

          <div class="flex justify-between items-center">
            <span>public</span>
            <span class="bg-zinc-800 px-2 rounded-full text-xs">0</span>
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

      <!-- Content Area -->
      <div class="flex-1">
        <!-- Content goes here -->
      </div>
    </div>
  </div>
</template>
