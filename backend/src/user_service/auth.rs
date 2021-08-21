use flowy_net::errors::NetworkError;
use flowy_user::{entities::SignUpResponse, protobuf::SignUpParams};
use sqlx::PgPool;
use std::sync::Arc;

pub struct Auth {
    db_pool: Arc<PgPool>,
}

impl Auth {
    pub fn new(db_pool: Arc<PgPool>) -> Self { Self { db_pool } }

    pub fn sign_up(&self, params: SignUpParams) -> Result<SignUpResponse, NetworkError> {
        // email exist?

        // generate user id

        //

        unimplemented!()
    }

    pub fn is_email_exist(&self, email: &str) -> bool {}
}
