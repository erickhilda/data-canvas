<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Button, FloatLabel, InputText, Select } from "primevue";

const dbType = ref("sqlite");
const dbTypeOptions = [
  { name: "SQLite", value: "sqlite" },
  { name: "PostgreSQL", value: "postgres" },
];
const dbName = ref("my_database");
const dbUrl = ref("");
const dbPath = ref("");
const connected = ref(false);
const tables = ref([]);

async function connectDB() {
  try {
    const pathOrUrl = dbType.value === "postgres" ? dbUrl.value : dbPath.value;
    await invoke("connect_database", {
      dbType: dbType.value,
      name: dbName.value,
      pathOrUrl,
    });
    connected.value = true;
    fetchTables();
  } catch (error) {
    console.error("Connection error:", error);
  }
}

async function fetchTables() {
  try {
    tables.value = await invoke("list_tables", {
      dbType: dbType.value,
      name: dbName.value,
    });
  } catch (error) {
    console.error("Error fetching tables:", error);
  }
}
</script>

<template>
  <div class="flex flex-col gap-4 p-6">
    <h1>Database Manager</h1>

    <div class="flex flex-col gap-4">
      <FloatLabel class="w-full md:w-56" variant="on">
        <Select
          v-model="dbType"
          :options="dbTypeOptions"
          option-label="name"
          option-value="value"
          placeholder="Select Database Type"
          class="w-full md:w-56"
          inputId="database_type"
        />
        <label for="database_type">Database Type</label>
      </FloatLabel>

      <FloatLabel variant="on">
        <label for="database_name">Database Name</label>
        <InputText id="database_name" v-model="dbName" />
      </FloatLabel>

      <FloatLabel variant="on" v-if="dbType === 'postgres'">
        <label for="database_url">PostgreSQL URL:</label>
        <InputText id="database_url" v-model="dbUrl" />
      </FloatLabel>

      <FloatLabel variant="on" v-if="dbType === 'sqlite'">
        <label for="database_path">SQLite file path</label>
        <InputText id="database_path" v-model="dbPath" />
      </FloatLabel>

      <Button @click="connectDB">Connect</Button>
    </div>

    <div v-if="connected">
      <h2>Tables</h2>
      <button @click="fetchTables">Refresh Tables</button>
      <!-- <ul> -->
      <!--   <li v-for="table in tables" :key="table">{{ table }}</li> -->
      <!-- </ul> -->
      <pre>{{ JSON.stringify(tables, null, 2) }}</pre>
    </div>
  </div>
</template>
