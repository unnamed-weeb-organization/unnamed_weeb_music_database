#[derive(Clone, Debug)]
pub struct Context {
	db: sqlx::postgres::PgPool,
}

impl juniper::Context for Context {}

impl Context {
	pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
		Self { db }
	}
}