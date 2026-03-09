use std::sync::Arc;

use sqlx::PgPool;

use crate::auth::token::TokenService;

#[derive(Clone)]
pub struct AppState {
    pool: Arc<PgPool>,
    token_service: TokenService,
}

impl AppState {
    pub fn new(pool: Arc<PgPool>) -> Self {
        let token_service = TokenService::new("".to_owned(), "".to_owned(), 59, i32::MAX as i64);
        Self {
            pool,
            token_service,
        }
    }

    pub fn get_pool(&self) -> &PgPool {
        self.pool.as_ref()
    }

    pub fn get_token_service(&self) -> &TokenService {
        &self.token_service
    }
}
