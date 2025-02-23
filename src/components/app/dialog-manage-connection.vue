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
import { ref } from "vue";
import FormConnectPostgres from "./form-connect-postgres.vue";

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
</script>

<template>
  <AppDialog title="Manage connection">
    <template #trigger>
      <DialogTrigger as-child>
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
          <div class="p-4">
            <div v-for="tag in tags" :key="tag">
              <div class="text-sm">
                {{ tag }}
              </div>
              <Separator class="my-2" />
            </div>
          </div>
        </ScrollArea>
      </div>

      <Separator orientation="vertical" class="mx-2" />

      <div class="flex-1">
        <component
          :is="databaseTypes[selectedDatabaseType]['component']"
        ></component>
      </div>
    </div>
  </AppDialog>
</template>
