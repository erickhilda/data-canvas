use crate::modules::database::DatabaseManager;
use sqlx::Row;

#[tauri::command]
pub async fn test_connection_sqlite(file_path: String) -> Result<String, String> {
    let url = format!("sqlite://{}", file_path);
    match sqlx::SqlitePool::connect(&url).await {
        Ok(_) => Ok("SQLite connection successful!".to_string()),
        Err(e) => Err(format!("SQLite connection failed: {}", e)),
    }
}

#[tauri::command]
pub async fn connect_sqlite(
    state: tauri::State<'_, DatabaseManager>,
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
pub async fn list_tables_sqlite(
    state: tauri::State<'_, DatabaseManager>,
    name: String,
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
