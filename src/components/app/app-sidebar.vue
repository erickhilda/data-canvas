<script setup lang="ts">
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { IconSelector, IconTable } from "@tabler/icons-vue";
import { DropdownMenuRoot } from "reka-ui";
import {
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "../ui/dropdown-menu";
import DialogManageConnection from "./dialog-manage-connection.vue";
import { useConnectionStore } from "@/stores/use-connection-store";
import { computed, ref } from "vue";
import { useInvoke } from "@/composables/use-invoke";

const connectionStore = useConnectionStore();

const activeConnection = computed(() => connectionStore.activeConnection);
const tables = computed(() =>
  connectionStore.tables.map((t) => ({ label: t, value: t })),
);

const activeSchema = ref("public");
const schemas = computed(() =>
  connectionStore.schemas.map((s) => ({ label: s, value: s })),
);

const { invokeRPC: invokeGetTablesBySchemaPostgres } = useInvoke(
  "get_tables_by_schema_postgres",
);

async function handleChangeSchema(schema: string) {
  try {
    const tables = await invokeGetTablesBySchemaPostgres<Array<string>>({
      name: activeConnection.value.name,
      schema,
    });

    connectionStore.setTables(tables);
    activeSchema.value = schema;
  } catch (error) {
    console.error(error);
  }
}
</script>

<template>
  <Sidebar>
    <SidebarHeader class="h-12 border-b">
      <SidebarGroupLabel class="text-base text-foreground">
        {{ activeConnection?.name || "Add connection" }}
      </SidebarGroupLabel>
      <DialogManageConnection />
    </SidebarHeader>

    <SidebarContent>
      <SidebarGroup>
        <SidebarMenu>
          <SidebarMenuItem>
            <DropdownMenuRoot>
              <DropdownMenuTrigger asChild>
                <SidebarMenuButton size="sm">
                  {{ activeSchema ?? "selected schema" }}
                  <IconSelector class="ml-auto" />
                </SidebarMenuButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                class="w-[--bits-dropdown-menu-anchor-width]"
              >
                <DropdownMenuItem v-for="schema in schemas" :key="schema.value">
                  <span
                    @click="() => handleChangeSchema(schema.value)"
                    class="w-full"
                  >
                    {{ schema.label }}
                  </span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenuRoot>
          </SidebarMenuItem>
        </SidebarMenu>
        <!-- <Separator class="my-2" /> -->
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="table in tables" :key="table.value">
              <SidebarMenuButton asChild class="cursor-pointer">
                <span class="flex gap-2">
                  <IconTable class="text-emerald-500" />
                  <span>{{ table.label }}</span>
                </span>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
  </Sidebar>
</template>
