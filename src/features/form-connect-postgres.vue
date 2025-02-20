<script setup lang="ts">
import { zodResolver } from "@primevue/forms/resolvers/zod";
import { z } from "zod";
import { Form, FormField, FormSubmitEvent } from "@primevue/forms";
import { InputText, Button, Password } from "primevue";
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useDbStore } from "../store/db-store";

const emit = defineEmits(["connected"]);

const initialValues = reactive({
  name: "New PostgreSQL Connection",
  host: "localhost",
  port: "5432",
  username: "postgres",
  password: "",
  databaseName: "postgres",
});
const resolver = zodResolver(
  z.object({
    name: z.string().min(1, { message: "Connection name is required." }),
    host: z.string().min(1, { message: "Host is required." }),
    port: z.string().min(1, { message: "Port is required" }),
    username: z.string().min(1, { message: "Username is required." }),
    password: z.string().optional(),
    databaseName: z.string().min(1, { message: "Database name is required" }),
  }),
);

const message = ref("");
const messageType = ref("info");

async function testConnection() {
  try {
    let response;
    response = await invoke("test_connection_postgres", {
      ...initialValues,
      port: +initialValues.port,
    });

    message.value = response as string;
    messageType.value = "success";
  } catch (error) {
    message.value = `Connection failed: ${error}`;
    messageType.value = "error";
  }
}
const dbStore = useDbStore();

async function connectToDatabase() {
  try {
    let response;
    response = await invoke("connect_postgres", {
      ...initialValues,
      port: +initialValues.port,
    });

    const tables = (await invoke("list_table_postgres", {
      name: initialValues.name,
    })) as Array<string>;

    dbStore.setConnection(initialValues.name, {
      ...initialValues,
      type: "postgres",
    });
    dbStore.setActiveConnection({ ...initialValues, type: "postgres" });
    dbStore.setTables(tables);

    emit("connected");
  } catch (error) {
    message.value = `Connection failed: ${error}`;
    messageType.value = "error";
  }
}

const onFormSubmit = async (e: FormSubmitEvent) => {
  if (e.valid) {
    await connectToDatabase();
  }
};
</script>

<template>
  <Form
    :resolver
    @submit="onFormSubmit"
    :initial-values="initialValues"
    class="flex flex-col justify-between gap-4 w-full h-full"
  >
    <div class="grid grid-cols-12 gap-y-4 gap-x-4">
      <FormField name="name" class="col-span-12 flex flex-col gap-1">
        <InputText size="small" fluid />
      </FormField>

      <FormField name="host" class="col-span-8 flex flex-col gap-1">
        <InputText size="small" placeholder="localhost" fluid />
      </FormField>

      <FormField name="port" class="col-span-4 flex flex-col gap-1">
        <InputText size="small" placeholder="port" fluid />
      </FormField>

      <FormField name="username" class="col-span-12 flex flex-col gap-1">
        <InputText size="small" fluid placeholder="Username" />
      </FormField>

      <FormField name="password" class="col-span-12 flex flex-col gap-1">
        <Password
          placeholder="Password"
          :feedback="false"
          toggleMask
          fluid
          size="small"
        />
      </FormField>

      <FormField name="databaseName" class="col-span-12 flex flex-col gap-1">
        <InputText size="small" fluid placeholder="Database name" />
      </FormField>
    </div>

    <div>
      <Message v-if="message" :severity="messageType">{{ message }}</Message>
    </div>
    <div class="flex justify-between">
      <Button
        type="submit"
        severity="secondary"
        size="small"
        label="Test"
        @click="testConnection"
      />

      <Button type="submit" label="Connect" size="small" />
    </div>
  </Form>
</template>
