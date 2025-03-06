use sqlx::{Pool, Postgres, Sqlite};
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct ConnectionManager {
    pub sqlite_pools: Mutex<HashMap<String, Pool<Sqlite>>>,
    pub postgres_pools: Mutex<HashMap<String, Pool<Postgres>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            sqlite_pools: Mutex::new(HashMap::new()),
            postgres_pools: Mutex::new(HashMap::new()),
        }
    }
}
