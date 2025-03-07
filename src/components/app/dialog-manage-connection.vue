<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { DialogTrigger } from "@/components/ui/dialog";
import { SidebarGroupAction } from "@/components/ui/sidebar";
import { IconDatabase } from "@tabler/icons-vue";
import AppDialog from "./app-dialog.vue";
import { ScrollArea } from "../ui/scroll-area";
import { Separator } from "../ui/separator";
import { DropdownMenuRoot } from "reka-ui";
import {
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "../ui/dropdown-menu";
import { computed, ref } from "vue";
import FormConnectPostgres from "./form-connect-postgres.vue";
import { useConnectionStore } from "@/stores/use-connection-store";

const tags = Array.from({ length: 50 }).map(
  (_, i, a) => `v1.2.0-beta.${a.length - i}`,
);

const databaseTypes = {
  postgres: {
    label: "PostgreSQL",
    value: "postgres",
    component: FormConnectPostgres,
    disabled: false,
  },
  mysql: {
    label: "MySQL",
    value: "mysql",
    component: null,
    disabled: true,
  },
  sqlite: {
    label: "SQLite",
    value: "sqlite",
    component: null,
    disabled: true,
  },
};
const selectedDatabaseType = ref("postgres");
const isManageConnectionDialogOpen = ref(false);

const connectionStore = useConnectionStore();

// const connections = computed(() => {
//   const connKeys = Object.keys(connectionStore.connections);
//   const conn = [];
//
//   connKeys.forEach((c) => {
//     conn.push(connectionStore.connections[c]);
//   });
// });

const connections = computed(() => connectionStore.connections);
</script>

<template>
  <AppDialog
    title="Manage connection"
    :open="isManageConnectionDialogOpen"
    @update:open="isManageConnectionDialogOpen = $event"
  >
    <template #trigger>
      <DialogTrigger class="cursor-pointer" as-child>
        <SidebarGroupAction title="Add new connection">
          <IconDatabase />
          <span class="sr-only">Add new connection</span>
        </SidebarGroupAction>
      </DialogTrigger>
    </template>

    <div class="flex items-start">
      <div class="flex flex-col gap-2">
        <DropdownMenuRoot>
          <DropdownMenuTrigger asChild>
            <Button size="sm" variant="outline" class="w-full">
              New Connection
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent class="w-[--bits-dropdown-menu-anchor-width]">
            <DropdownMenuItem
              v-for="d in databaseTypes"
              @select="() => (selectedDatabaseType = d.value)"
              :disabled="d.disabled"
            >
              <span>{{ d.label }}</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenuRoot>

        <ScrollArea class="min-h-[340px] h-[340px] max-h-96 w-40">
          <div
            v-for="conn in connections"
            :key="conn.name"
            class="hover:bg-border/50 rounded-lg w-full border border-dashed p-2 mt-2 cursor-pointer"
          >
            <div class="text-sm">
              {{ conn.name }}
            </div>
          </div>
        </ScrollArea>
      </div>

      <Separator orientation="vertical" class="mx-2" />

      <div class="flex-1">
        <component
          :is="databaseTypes[selectedDatabaseType]['component']"
          @success-connect="isManageConnectionDialogOpen = false"
        ></component>
      </div>
    </div>
  </AppDialog>
</template>
