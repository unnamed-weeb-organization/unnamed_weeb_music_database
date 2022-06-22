use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
	pub db: Arc<sqlx::postgres::PgPool>,
}

impl Context {
	pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
		Self { db: Arc::new(db) }
	}
}