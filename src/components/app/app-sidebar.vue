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
import {
  IconCalendar,
  IconSelector,
  IconHome,
  IconInbox,
  IconSearch,
  IconSettings,
} from "@tabler/icons-vue";
import { DropdownMenuRoot } from "reka-ui";
import {
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "../ui/dropdown-menu";
import { Separator } from "../ui/separator";
import DialogManageConnection from "./dialog-manage-connection.vue";

// Menu items.
const items = [
  {
    title: "Home",
    url: "#",
    icon: IconHome,
  },
  {
    title: "Inbox",
    url: "#",
    icon: IconInbox,
  },
  {
    title: "Calendar",
    url: "#",
    icon: IconCalendar,
  },
  {
    title: "Search",
    url: "#",
    icon: IconSearch,
  },
  {
    title: "Settings",
    url: "#",
    icon: IconSettings,
  },
];
</script>

<template>
  <Sidebar>
    <SidebarHeader class="h-12 border-b">
      <SidebarGroupLabel class="text-base text-foreground">
        Active connection
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
                  selected schema
                  <IconSelector class="ml-auto" />
                </SidebarMenuButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                class="w-[--bits-dropdown-menu-anchor-width]"
              >
                <DropdownMenuItem>
                  <span>Acme Inc</span>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <span>Acme Corp.</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenuRoot>
          </SidebarMenuItem>
        </SidebarMenu>
        <!-- <Separator class="my-2" /> -->
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in items" :key="item.title">
              <SidebarMenuButton asChild>
                <a :href="item.url">
                  <component :is="item.icon" />
                  <span>{{ item.title }}</span>
                </a>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
  </Sidebar>
</template>
