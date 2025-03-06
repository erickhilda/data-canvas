mod modules;

use modules::connection::ConnectionManager;
use modules::{postgres, sqlite};

pub fn run() {
    tauri::Builder::default()
        .manage(ConnectionManager::new())
        //.plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            postgres::test_connection_postgres,
            postgres::list_tables_postgres,
            postgres::connect_postgres,
            postgres::get_schemas_postgres,
            postgres::get_tables_by_schema_postgres,
            sqlite::test_connection_sqlite,
            sqlite::connect_sqlite,
            sqlite::list_tables_sqlite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
