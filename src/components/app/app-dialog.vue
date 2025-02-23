<script setup lang="ts">
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";

const props = defineProps<{
  title: string;
}>();
</script>

<template>
  <Dialog>
    <slot name="trigger" />

    <DialogContent class="sm:max-w-xl">
      <DialogHeader>
        <DialogTitle>{{ props.title }}</DialogTitle>
      </DialogHeader>
      <slot />

      <slot name="footer" />
    </DialogContent>
  </Dialog>
</template>
