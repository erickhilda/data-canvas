import { defineStore } from "pinia";

export const useDbStore = defineStore("db", {
  state: () => ({
    activeConnection: {} as any,
    connection: {} as Record<string, any>,
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
    setTables(tables: string[]) {
      this.tables = tables;
    },
  },
});
