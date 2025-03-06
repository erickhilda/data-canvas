<script setup lang="ts">
import type { Component, HTMLAttributes } from "vue";
import { cn } from "@/lib/classnames";
import { useVModel } from "@vueuse/core";
import { Button } from "../button";

const props = defineProps<{
  defaultValue?: string | number;
  modelValue?: string | number;
  class?: HTMLAttributes["class"];
  leftSection?: Component;
  rightSection?: Component;
  type: HTMLInputElement["type"];
  disabled?: HTMLInputElement["disabled"];
}>();

const emits = defineEmits<{
  (e: "update:modelValue", payload: string | number): void;
  (e: "clickRightSection"): void;
}>();

const modelValue = useVModel(props, "modelValue", emits, {
  passive: true,
  defaultValue: props.defaultValue,
});
</script>

<template>
  <div class="relative w-full max-w-sm items-center">
    <span
      v-if="leftSection"
      class="absolute start-0 inset-y-0 flex items-center justify-center px-2"
    >
      <Button type="button" size="icon-sm" variant="ghost">
        <component :is="props.leftSection" />
      </Button>
    </span>
    <input
      v-model="modelValue"
      :class="
        cn(
          'flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:bg-sidebar',
          props.class,
          rightSection ? 'pr-8' : '',
          leftSection ? 'pl-8' : '',
        )
      "
      :type="props.type ?? 'text'"
      :disabled="props.disabled"
    />
    <span
      v-if="rightSection"
      class="absolute end-0 inset-y-0 flex items-center justify-center px-2"
    >
      <Button
        type="button"
        size="icon-sm"
        variant="ghost"
        @click="emits('clickRightSection')"
      >
        <component :is="rightSection" />
      </Button>
    </span>
  </div>
</template>
