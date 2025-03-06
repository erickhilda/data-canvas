<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import Button from "@/components/ui/button/Button.vue";
import { useInvoke } from "@/composables/useInvoke";
import { useToast } from "@/components/ui/toast/use-toast";
import InputText from "./form/input-text.vue";
import InputPassword from "./form/input-password.vue";
import { useConnectionStore } from "@/stores/use-connection-store";
import { computed } from "vue";

const { toast } = useToast();
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
    password: z.string().optional().default(""),
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

const { isLoading: isTestingConnection, invokeRPC: invokeTestConnection } =
  useInvoke("test_connection_postgres");
const testConnection = async () => {
  try {
    const validateRes = await validate();
    if (validateRes.valid) {
      const response = await invokeTestConnection<string>({
        ...formValues,
        port: +(formValues?.port ?? ""),
      });

      toast({
        description: response,
      });
    }
  } catch (error) {
    toast({
      variant: "destructive",
      description: error as string,
    });
  }
};

const connectionStore = useConnectionStore();

const { isLoading: isConnecting, invokeRPC: invokeConnectPostgres } =
  useInvoke("connect_postgres");
const onSubmit = handleSubmit(async (values) => {
  try {
    await invokeConnectPostgres({
      ...values,
      port: +values.port,
    });

    connectionStore.setConnection(values.name ?? "", {
      ...values,
      type: "postgres",
    });
  } catch (error) {
    toast({
      variant: "destructive",
      description: error as string,
    });
  }
});
const isDisabled = computed(() => isConnecting || isTestingConnection);
</script>

<template>
  <form class="grid grid-cols-12 gap-2" @submit="onSubmit" aria-disabled="true">
    <InputText
      class="col-span-12"
      name="name"
      label="Connection name"
      :disabled="isDisabled.value"
    />
    <InputText
      class="col-span-8"
      name="host"
      label="Host"
      :disabled="isDisabled.value"
    />
    <InputText
      class="col-span-4"
      name="port"
      label="Port"
      :disabled="isDisabled.value"
    />

    <InputText
      class="col-span-12"
      name="username"
      label="Username"
      :disabled="isDisabled.value"
    />
    <InputPassword
      class="col-span-12"
      name="password"
      label="Password"
      :disabled="isDisabled.value"
    />
    <InputText
      class="col-span-12"
      name="databaseName"
      label="Database name"
      :disabled="isDisabled.value"
    />

    <div class="col-span-12"></div>
    <div class="col-span-12 flex justify-between">
      <Button
        type="button"
        size="sm"
        variant="secondary"
        @click="testConnection"
        :loading="isTestingConnection"
        :disabled="isDisabled.value"
      >
        Test
      </Button>
      <Button
        type="submit"
        size="sm"
        :loading="isConnecting"
        :disabled="isDisabled.value"
      >
        Submit
      </Button>
    </div>
  </form>
</template>
