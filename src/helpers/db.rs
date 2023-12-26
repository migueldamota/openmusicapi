use lazy_static::lazy_static;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

const DATABASE_URL: &str = "postgres://postgres:root@127.0.0.1/openmusicapi";

lazy_static! {
    static ref POOL: Pool = {
        let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
        Pool::new(manager).expect("Failed to create db pool.")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let _conn = connection().expect("Failed to get db connection");
    // embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, Error> {
    POOL.get()
        .map_err(|e| panic!("Database connection error: {}", e))
        // .map_err(|e| Error::new(ErrorKind::Other, format!("Failed getting db connection: {}", e)))
}
