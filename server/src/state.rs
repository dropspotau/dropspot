use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    auth::token::TokenService,
    config::{ServerConfiguration, get_server_config},
};

#[derive(Clone)]
pub struct AppState {
    pool: Arc<PgPool>,
    token_service: TokenService,
    server_config: ServerConfiguration,
}

impl AppState {
    pub fn new(pool: Arc<PgPool>) -> Self {
        let token_service = TokenService::new("".to_owned(), "".to_owned(), 59, 600000);
        let config = get_server_config();

        Self {
            pool,
            token_service,
            server_config: config,
        }
    }

    pub fn get_pool(&self) -> &PgPool {
        self.pool.as_ref()
    }

    pub fn get_token_service(&self) -> &TokenService {
        &self.token_service
    }

    pub fn get_server_config(&self) -> &ServerConfiguration {
        &self.server_config
    }
}
