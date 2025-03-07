import { invoke, InvokeArgs, InvokeOptions } from "@tauri-apps/api/core";
import { ref } from "vue";

export function useInvoke(functionName: string) {
  const isLoading = ref(false);

  async function invokeRPC<T>(args?: InvokeArgs, options?: InvokeOptions) {
    try {
      isLoading.value = true;
      const response = await invoke(functionName, args, options);

      return response as T;
    } catch (error) {
      throw error;
    } finally {
      isLoading.value = false;
    }
  }
  return {
    isLoading,
    invokeRPC,
  };
}
