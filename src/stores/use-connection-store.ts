import { defineStore } from "pinia";

export const useConnectionStore = defineStore("connection", {
  state: () => ({
    activeConnection: {} as any,
    connections: {} as Record<string, any>,
    schemas: [] as string[],
    tables: [] as string[],
  }),
  actions: {
    setActiveConnection(conn: any) {
      this.activeConnection = conn;
    },
    addConnection(name: string, conn: any) {
      this.connections = {
        ...this.connections,
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
