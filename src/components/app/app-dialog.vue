<script setup lang="ts">
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { computed } from "vue";

const props = defineProps<{
  title: string;
  open: boolean;
}>();

const emits = defineEmits(["update:open"]);

const isOpen = computed({
  get: () => props.open,
  set: (value) => emits("update:open", value),
});
</script>

<template>
  <Dialog v-model:open="isOpen">
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
