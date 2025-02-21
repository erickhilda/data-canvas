use crate::modules::database::DatabaseManager;
use sqlx::Row;

#[tauri::command]
pub async fn test_connection_postgres(
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
pub async fn connect_postgres(
    state: tauri::State<'_, DatabaseManager>,
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
pub async fn list_tables_postgres(
    state: tauri::State<'_, DatabaseManager>,
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
