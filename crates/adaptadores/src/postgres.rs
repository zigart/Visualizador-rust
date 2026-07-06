use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresState {
    pool: Arc<PgPool>,
}

impl PostgresState {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
