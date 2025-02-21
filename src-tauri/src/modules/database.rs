use sqlx::{Pool, Postgres, Sqlite};
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct DatabaseManager {
    pub sqlite_pools: Mutex<HashMap<String, Pool<Sqlite>>>,
    pub postgres_pools: Mutex<HashMap<String, Pool<Postgres>>>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            sqlite_pools: Mutex::new(HashMap::new()),
            postgres_pools: Mutex::new(HashMap::new()),
        }
    }
}
