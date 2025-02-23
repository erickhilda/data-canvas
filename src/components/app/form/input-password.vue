<script setup lang="ts">
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import Input from "@/components/ui/input/Input.vue";
import { IconEye, IconEyeClosed } from "@tabler/icons-vue";
import type { Component, HTMLAttributes } from "vue";
import { computed, ref } from "vue";

const props = defineProps<{
  name: string;
  label: string;
  placeholder?: string;
  leftSection?: Component;
  rightSection?: Component;
  class?: HTMLAttributes["class"];
}>();

const inputType = ref("password");
const changeInputType = () => {
  if (inputType.value === "password") {
    inputType.value = "text";
  } else {
    inputType.value = "password";
  }
};

const rightAction = computed(() => {
  if (inputType.value === "password") {
    return IconEyeClosed;
  }

  return IconEye;
});
</script>

<template>
  <FormField v-slot="{ componentField }" :name="props.name">
    <FormItem :class="props.class">
      <FormLabel>{{ props.label }}</FormLabel>
      <FormControl>
        <Input
          :type="inputType"
          :placeholder="props.placeholder ?? ''"
          v-bind="componentField"
          :left-section="leftSection"
          :right-section="rightSection ?? rightAction"
          @click-right-section="changeInputType"
        />
      </FormControl>
      <!-- <FormMessage /> -->
    </FormItem>
  </FormField>
</template>
