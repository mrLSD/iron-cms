use iron_diesel_middleware::{DieselMiddleware};
use diesel::result::Error;
use r2d2;
use r2d2_diesel;
use diesel::pg::PgConnection;

pub type ConnectionPool = r2d2::PooledConnection<r2d2_diesel::ConnectionManager<PgConnection>>;
pub type InsertResult = Result<usize, Error>;

/// Create DB connection
pub fn db(connection_url: &str) -> DieselMiddleware {
    DieselMiddleware::new(connection_url).unwrap()
}
