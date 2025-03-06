import { defineStore } from "pinia";

export const useConnectionStore = defineStore("db", {
  state: () => ({
    activeConnection: {} as any,
    connection: {} as Record<string, any>,
    schemas: [] as string[],
    tables: [] as string[],
  }),
  actions: {
    setActiveConnection(conn: any) {
      this.activeConnection = conn;
    },
    setConnection(name: string, conn: any) {
      this.connection = {
        ...this.connection,
        [name]: conn,
      };
    },
    setSchemas(schemas: string[]) {
      this.schemas = schemas;
    },
    setTables(tables: string[]) {
      this.tables = tables;
    },
  },
});
