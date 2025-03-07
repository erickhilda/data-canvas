<script setup lang="ts">
import AppSidebar from "@/components/app/app-sidebar.vue";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import AppCanvas from "@/components/app/app-canvas.vue";
import Toaster from "@/components/ui/toast/Toaster.vue";
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <SidebarInset>
      <header class="flex h-12 shrink-0 items-center gap-2 border-b px-4">
        <SidebarTrigger />
      </header>

      <Toaster />
      <AppCanvas />
    </SidebarInset>
  </SidebarProvider>
</template>
