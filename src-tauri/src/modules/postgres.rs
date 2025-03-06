use crate::modules::connection::ConnectionManager;
use serde::Serialize;
use sqlx::Row;
use tauri::State;

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

#[derive(Serialize)]
pub struct ConnectionResponse {
    name: String,
    message: String,
    url: String,
}

#[tauri::command]
pub async fn connect_postgres(
    state: State<'_, ConnectionManager>,
    name: String,
    host: String,
    port: u16,
    username: String,
    password: String,
    database_name: String,
) -> Result<ConnectionResponse, String> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, database_name
    );
    match sqlx::PgPool::connect(&url).await {
        Ok(pool) => {
            state.postgres_pools.lock().await.insert(name.clone(), pool);
            Ok(ConnectionResponse {
                name,
                message: "Successfully connected to PostgreSQL".to_string(),
                url,
            })
        }
        Err(e) => Err(format!("Failed to connect to PostgreSQL: {}", e)),
    }
}

#[tauri::command]
pub async fn get_schemas_postgres(
    state: State<'_, ConnectionManager>,
    name: String,
) -> Result<Vec<String>, String> {
    let pools = state.postgres_pools.lock().await;
    let pool = pools.get(&name).ok_or("Database connection not found")?;

    let query = "SELECT schema_name FROM information_schema.schemata WHERE schema_name NOT LIKE 'pg_%' AND schema_name != 'information_schema'";
    match sqlx::query(query).fetch_all(pool).await {
        Ok(rows) => {
            let schemas: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
            Ok(schemas)
        }
        Err(e) => Err(format!("Failed to get schemas: {}", e)),
    }
}

#[tauri::command]
pub async fn get_tables_by_schema_postgres(
    state: State<'_, ConnectionManager>,
    name: String,
    schema: String,
) -> Result<Vec<String>, String> {
    let pools = state.postgres_pools.lock().await;
    let pool = pools.get(&name).ok_or("Database connection not found")?;

    let query = "SELECT table_name FROM information_schema.tables WHERE table_schema = $1";
    match sqlx::query(query).bind(&schema).fetch_all(pool).await {
        Ok(rows) => {
            let tables: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
            Ok(tables)
        }
        Err(e) => Err(format!("Failed to get tables for schema {}: {}", schema, e)),
    }
}

#[tauri::command]
pub async fn list_tables_postgres(
    state: State<'_, ConnectionManager>,
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
