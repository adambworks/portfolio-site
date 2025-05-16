use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> DBPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool")
}