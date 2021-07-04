use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool(db_url: &str) -> Pool {
    // set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}