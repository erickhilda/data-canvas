<script setup lang="ts">
import { Button, Dialog, Popover, ScrollPanel } from "primevue";
import { computed, ref } from "vue";
import FormConnectPostgres from "./form-connect-postgres.vue";
import FormConnectSqlite from "./form-connect-sqlite.vue";
import { useDbStore } from "../store/db-store";

const databaseTypes = [
  {
    label: "PostgreSQL",
    value: "postgres",
  },
  {
    label: "SQLite",
    value: "sqlite",
  },
];
const databaseTypeMenu = ref();
const toggleDatabaseTypeMenu = (event: any) => {
  databaseTypeMenu.value.toggle(event);
};
const selectedDatabaseType = ref("postgres");
const selectDatabaseType = (value: string) => {
  selectedDatabaseType.value = value;
  databaseTypeMenu.value.hide();
};

const isModalManageConnectionOpen = ref(true);

const dbStore = useDbStore();

const connection = computed(() => dbStore.connection);
</script>

<template>
  <div class="card flex justify-center">
    <Button
      icon="pi pi-database"
      text
      @click="isModalManageConnectionOpen = !isModalManageConnectionOpen"
    />
    <Dialog
      v-model:visible="isModalManageConnectionOpen"
      modal
      header="Manage Connection"
      :style="{ width: '36rem' }"
      :pt="{
        header: '!py-0 px-2 text-base border-b border-zinc-700',
        pcCloseButton: '!p-2',
        content: '!p-0 !overflow-hidden',
        title: '!text-base !font-semibold',
      }"
    >
      <div class="flex px-3 w-full">
        <div
          class="border-r border-zinc-700 flex flex-col gap-3 pr-3 py-3 w-48"
        >
          <div class="flex justify-center">
            <Button size="small" @click="toggleDatabaseTypeMenu" fluid>
              New Connection
            </Button>
            <Popover ref="databaseTypeMenu">
              <div class="flex flex-col gap-4">
                <ul class="list-none p-0 m-0 flex flex-col">
                  <li
                    v-for="dbType in databaseTypes"
                    :key="dbType.value"
                    class="flex items-center gap-2 px-2 py-3 hover:bg-emphasis cursor-pointer rounded-border"
                    @click="selectDatabaseType(dbType.value)"
                  >
                    <div class="text-sm text-surface-500 dark:text-surface-400">
                      {{ dbType.label }}
                    </div>
                  </li>
                </ul>
              </div>
            </Popover>
          </div>
          <ScrollPanel class="h-74">
            <div
              class="w-full p-2 border border-zinc-700 mt-2 rounded text-sm truncate"
              v-for="i in Object.keys(connection)"
            >
              {{ i }}
            </div>
          </ScrollPanel>
        </div>

        <div class="w-full pl-3 py-3">
          <FormConnectPostgres
            v-if="selectedDatabaseType === 'postgres'"
            @connected="
              isModalManageConnectionOpen = !isModalManageConnectionOpen
            "
          />

          <FormConnectSqlite v-if="selectedDatabaseType === 'sqlite'" />
        </div>
      </div>
    </Dialog>
  </div>
</template>
