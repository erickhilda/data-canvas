use sqlx::{Pool, Postgres, Row, Sqlite};
use std::collections::HashMap;
use tokio::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct DatabaseManager {
    sqlite_pools: Mutex<HashMap<String, Pool<Sqlite>>>,
    //mysql_poools: Mutex<HashMap<String, Pool<MySql>>>
    pg_pools: Mutex<HashMap<String, Pool<Postgres>>>,
}

impl DatabaseManager {
    async fn add_sqlite(&self, name: String, path: String) -> Result<(), String> {
        let pool = sqlx::SqlitePool::connect(&format!("sqlite://{}", path))
            .await
            .map_err(|e| e.to_string())?;
        self.sqlite_pools.lock().await.insert(name, pool);
        Ok(())
    }

    async fn add_postgres(&self, name: String, url: String) -> Result<(), String> {
        let pool = sqlx::PgPool::connect(&url)
            .await
            .map_err(|e| e.to_string())?;
        self.pg_pools.lock().await.insert(name, pool);
        Ok(())
    }
}

#[tauri::command]
async fn connect_database(
    db_type: String,
    name: String,
    path_or_url: String,
    state: tauri::State<'_, DatabaseManager>,
) -> Result<(), String> {
    match db_type.as_str() {
        "sqlite" => state.inner().add_sqlite(name, path_or_url).await,
        "postgres" => state.inner().add_postgres(name, path_or_url).await,
        _ => Err("Database type not supported".into()),
    }
}

#[tauri::command]
async fn list_tables(
    db_type: String,
    name: String,
    state: tauri::State<'_, DatabaseManager>,
) -> Result<Vec<String>, String> {
    match db_type.as_str() {
        "sqlite" => {
            let pools = state.sqlite_pools.lock().await;
            if let Some(pool) = pools.get(&name) {
                let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='table';")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(rows.into_iter().map(|row| row.get(0)).collect())
            } else {
                Err("Database not found".into())
            }
        }
        "postgres" => {
            let pools = state.pg_pools.lock().await;
            if let Some(pool) = pools.get(&name) {
                let rows = sqlx::query(
                    "SELECT table_name FROM information_schema.tables WHERE table_schema='public';",
                )
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(rows.into_iter().map(|row| row.get(0)).collect())
            } else {
                Err("Database not found".into())
            }
        }
        _ => Err("Database type not supported".into()),
    }
}

//fn main() {
//    tauri::Builder::default()
//        .manage(DatabaseManager {
//            sqlite_pools: Mutex::new(HashMap::new()),
//            //mysql_pools: Mutex::new(HashMap::new()),
//            pg_pools: Mutex::new(HashMap::new()),
//        })
//        .invoke_handler(tauri::generate_handler![connect_database])
//        .run(tauri::generate_context!())
//        .expect("failed to run app");
//}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DatabaseManager {
            sqlite_pools: Mutex::new(HashMap::new()),
            //mysql_pools: Mutex::new(HashMap::new()),
            pg_pools: Mutex::new(HashMap::new()),
        })
        //.plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            connect_database,
            list_tables
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
