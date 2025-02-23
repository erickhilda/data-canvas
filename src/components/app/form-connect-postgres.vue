<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import InputText from "./form/input-text.vue";
import InputPassword from "./form/input-password.vue";
import { Button } from "../ui/button";

const formSchema = toTypedSchema(
  z.object({
    name: z.string().optional(),
    host: z
      .string({ required_error: "Host is required" })
      .min(1, "Host is required"),
    port: z
      .string({ required_error: "Port is required" })
      .min(1, "Port is required"),
    username: z
      .string({ required_error: "Username is required" })
      .min(1, "Username is required"),
    password: z.string().optional(),
    databaseName: z
      .string({ required_error: "Username is required" })
      .min(1, "Username is required"),
  }),
);

const {
  handleSubmit,
  validate,
  values: formValues,
} = useForm({
  validationSchema: formSchema,
});

const testConnection = async () => {
  const validateRes = await validate();
  if (validateRes.valid) {
    console.log(validateRes);
    return;
  }
  console.log("values", formValues);
};

const onSubmit = handleSubmit((values) => {
  console.log("Form submitted!", values);
});
</script>

<template>
  <form class="grid grid-cols-12 gap-2" @submit="onSubmit">
    <InputText class="col-span-12" name="name" label="Connection name" />
    <InputText class="col-span-8" name="host" label="Host" />
    <InputText class="col-span-4" name="port" label="Port" />

    <InputText class="col-span-12" name="username" label="Username" />
    <InputPassword class="col-span-12" name="password" label="Password" />
    <InputText class="col-span-12" name="databaseName" label="Database name" />

    <div class="col-span-12"></div>
    <div class="col-span-12 flex justify-between">
      <Button
        type="button"
        size="sm"
        variant="secondary"
        @click="testConnection"
      >
        Test
      </Button>
      <Button type="submit" size="sm"> Submit </Button>
    </div>
  </form>
</template>
