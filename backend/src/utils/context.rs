use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Context {
	pub db: Arc<sqlx::postgres::PgPool>,
}

impl juniper::Context for Context {}

impl Context {
	pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
		Self { db: Arc::new(db) }
	}
}