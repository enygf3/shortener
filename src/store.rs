use rand::{distr::Alphanumeric, RngExt};
use sqlx::PgPool;

#[derive(Clone)]
pub(crate) struct Store {
    pool: PgPool
}

impl Store {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn get_url(&self, url: &str) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar(
            "UPDATE links SET clicks = clicks + 1 WHERE code = $1 RETURNING url",
        )
            .bind(url)
            .fetch_optional(&self.pool)
            .await
    }

    pub(crate) async fn put_url(&self, url: &str) -> Result<String, sqlx::Error> {
        loop {
            let code = Self::create_random_url();
            let res = sqlx::query("INSERT INTO links (code, url) VALUES ($1, $2)")
                .bind(&code)
                .bind(url)
                .execute(&self.pool)
                .await;

            match res {
                Ok(_) => return Ok(code),
                Err(sqlx::Error::Database(e)) if e.is_unique_violation() => continue,
                Err(e) => return Err(e),
            }
        }
    }

    fn create_random_url() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    }
}