<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/lib/classnames";
import { Primitive, type PrimitiveProps } from "reka-ui";
import { type ButtonVariants, buttonVariants } from ".";
import { IconLoader2 } from "@tabler/icons-vue";

interface Props extends PrimitiveProps {
  variant?: ButtonVariants["variant"];
  size?: ButtonVariants["size"];
  class?: HTMLAttributes["class"];
  loading?: boolean;
  disabled?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  as: "button",
  loading: false,
  disabled: false,
});
</script>

<template>
  <Primitive
    :as="as"
    :as-child="asChild"
    :class="cn(buttonVariants({ variant, size }), props.class)"
  >
    <IconLoader2 v-if="loading" />
    <slot v-else />
  </Primitive>
</template>
