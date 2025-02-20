use sqlx::{Pool, Postgres, Row, Sqlite};
use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
struct DatabaseManager {
    sqlite_pools: Mutex<HashMap<String, Pool<Sqlite>>>,
    postgres_pools: Mutex<HashMap<String, Pool<Postgres>>>,
}

#[tauri::command]
async fn test_connection_postgres(
    host: String,
    port: u16,
    username: String,
    password: String,
    database_name: String,
) -> Result<String, String> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, database_name
    );
    match sqlx::PgPool::connect(&url).await {
        Ok(_) => Ok("PostgreSQL connection successful!".to_string()),
        Err(e) => Err(format!("PostgreSQL connection failed: {}", e)),
    }
}

#[tauri::command]
async fn connect_postgres(
    state: State<'_, DatabaseManager>,
    name: String,
    host: String,
    port: u16,
    username: String,
    password: String,
    database_name: String,
) -> Result<String, String> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, database_name
    );
    match sqlx::PgPool::connect(&url).await {
        Ok(pool) => {
            state.postgres_pools.lock().await.insert(name.clone(), pool);
            Ok(format!("Connected to PostgreSQL: {}", name))
        }
        Err(e) => Err(format!("Failed to connect to PostgreSQL: {}", e)),
    }
}

#[tauri::command]
async fn list_table_postgres(
    state: State<'_, DatabaseManager>,
    name: String,
) -> Result<Vec<String>, String> {
    let pools = state.postgres_pools.lock().await;
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

// SQLite section
#[tauri::command]
async fn test_sqlite_connection(file_path: String) -> Result<String, String> {
    let url = format!("sqlite://{}", file_path);
    match sqlx::SqlitePool::connect(&url).await {
        Ok(_) => Ok("SQLite connection successful!".to_string()),
        Err(e) => Err(format!("SQLite connection failed: {}", e)),
    }
}

#[tauri::command]
async fn connect_sqlite(
    state: State<'_, DatabaseManager>,
    name: String,
    file_path: String,
) -> Result<String, String> {
    let url = format!("sqlite://{}", file_path);
    match sqlx::SqlitePool::connect(&url).await {
        Ok(pool) => {
            state.sqlite_pools.lock().await.insert(name.clone(), pool);
            Ok(format!("Connected to SQLite: {}", name))
        }
        Err(e) => Err(format!("Failed to connect to SQLite: {}", e)),
    }
}

#[tauri::command]
async fn list_table_sqlite(
    name: String,
    state: tauri::State<'_, DatabaseManager>,
) -> Result<Vec<String>, String> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DatabaseManager {
            sqlite_pools: Mutex::new(HashMap::new()),
            postgres_pools: Mutex::new(HashMap::new()),
        })
        //.plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            test_connection_postgres,
            list_table_postgres,
            connect_postgres,
            test_sqlite_connection,
            connect_sqlite,
            list_table_sqlite
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
