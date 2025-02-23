<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/lib/classnames";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();
</script>

<template>
  <div
    :class="
      cn(
        'flex flex-col gap-y-1.5 text-center sm:text-left pb-3 border-b -mx-3 px-3',
        props.class,
      )
    "
  >
    <slot />
  </div>
</template>
