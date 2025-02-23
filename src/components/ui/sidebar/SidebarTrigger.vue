<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/lib/classnames";
import { Button } from "@/components/ui/button";
import { IconLayoutSidebarFilled } from "@tabler/icons-vue";
import { useSidebar } from "./utils";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();

const { toggleSidebar } = useSidebar();
</script>

<template>
  <Button
    data-sidebar="trigger"
    variant="ghost"
    size="icon"
    :class="cn('h-7 w-7', props.class)"
    @click="toggleSidebar"
  >
    <IconLayoutSidebarFilled />
    <span class="sr-only">Toggle Sidebar</span>
  </Button>
</template>
