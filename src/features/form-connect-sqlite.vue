<!-- <div v-if="dbType.value === 'sqlite'" class="form-group"> -->
<!--       <label for="filePath">Database File</label> -->
<!--       <InputText id="filePath" v-model="filePath" placeholder="path/to/database.sqlite" /> -->
<!--       <Button label="Browse" icon="pi pi-folder-open" class="p-button-secondary" @click="selectFile" /> -->
<!--     </div> -->

<script setup lang="ts">
import { zodResolver } from "@primevue/forms/resolvers/zod";
import { z } from "zod";
import { Form, FormField } from "@primevue/forms";
import { InputText, Message, Button } from "primevue";

const resolver = zodResolver(
  z.object({
    username: z.string().min(1, { message: "Username is required." }),
    password: z.string().min(1, { message: "Password is required." }),
  }),
);

async function selectFile() {
  filePath.value = await open({ multiple: false });
}

const onFormSubmit = ({ valid }) => {
  if (valid) {
    console.log(valid);
  }
};
</script>

<template>
  <Form
    :resolver
    @submit="onFormSubmit"
    class="flex flex-col justify-between gap-4 w-full h-full"
  >
    <div class="grid grid-cols-8 gap-y-4 gap-x-4">
      <FormField
        v-slot="$field"
        name="connectionName"
        initialValue="New SQLite Connection"
        class="col-span-12 flex flex-col gap-1"
      >
        <InputText size="small" fluid />
        <Message
          v-if="$field?.invalid"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ $field.error?.message }}
        </Message>
      </FormField>

      <FormField
        v-slot="$field"
        name="databasePath"
        initialValue=""
        class="col-span-9 flex flex-col gap-1"
      >
        <InputText size="small" fluid />
        <Message
          v-if="$field?.invalid"
          severity="error"
          size="small"
          variant="simple"
        >
          {{ $field.error?.message }}
        </Message>
      </FormField>

      <Button
        type="button"
        severity="secondary"
        size="small"
        label="Browse"
        class="col-span-3"
      />
    </div>

    <div class="flex justify-between">
      <Button type="button" severity="secondary" size="small" label="Test" />

      <Button type="submit" label="Connect" size="small" />
    </div>
  </Form>
</template>
