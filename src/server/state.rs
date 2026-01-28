use sqlx::PgPool;

pub struct State {
    pool: PgPool,
}

impl State {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
