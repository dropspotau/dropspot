use sqlx::PgPool;

use crate::auth::token::TokenService;

pub struct AppState {
    pool: PgPool,
    token_service: TokenService,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let token_service = TokenService::new("".to_owned(), "".to_owned(), 60, i64::MAX);
        Self {
            pool,
            token_service,
        }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn get_token_service(&self) -> &TokenService {
        &self.token_service
    }
}
