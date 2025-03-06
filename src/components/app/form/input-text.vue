<script setup lang="ts">
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import Input from "@/components/ui/input/Input.vue";
import type { Component, HTMLAttributes } from "vue";

const props = withDefaults(
  defineProps<{
    name: string;
    label: string;
    placeholder?: string;
    leftSection?: Component;
    rightSection?: Component;
    class?: HTMLAttributes["class"];
    disabled?: boolean;
  }>(),
  {},
);
</script>

<template>
  <FormField v-slot="{ componentField }" :name="props.name">
    <FormItem :class="props.class">
      <FormLabel>{{ props.label }}</FormLabel>
      <FormControl>
        <Input
          size="sm"
          type="text"
          :placeholder="props.placeholder ?? ''"
          v-bind="componentField"
          :left-section="leftSection"
          :right-section="rightSection"
          :disabled="disabled"
        />
      </FormControl>
      <!-- <FormMessage /> -->
    </FormItem>
  </FormField>
</template>
